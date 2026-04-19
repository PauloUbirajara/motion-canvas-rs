use motion_canvas_rs::prelude::*;
use std::time::Duration;

const BG: Color = Color::rgb8(0x0e, 0x0e, 0x12);
const WHITE: Color = Color::rgb8(0xf0, 0xf0, 0xf0);
const DIM: Color = Color::rgb8(0x55, 0x55, 0x66);
const ACCENT: Color = Color::rgb8(0x68, 0xab, 0xdf);
const YELLOW: Color = Color::rgb8(0xe6, 0xa7, 0x00);
const GREEN: Color = Color::rgb8(0x25, 0xc2, 0x81);
const RED: Color = Color::rgb8(0xe1, 0x32, 0x38);

trait HasOpacity {
    fn opacity_signal(&self) -> Signal<f32>;
}
impl HasOpacity for TextNode {
    fn opacity_signal(&self) -> Signal<f32> {
        self.opacity.clone()
    }
}
impl HasOpacity for Circle {
    fn opacity_signal(&self) -> Signal<f32> {
        self.opacity.clone()
    }
}
impl HasOpacity for Rect {
    fn opacity_signal(&self) -> Signal<f32> {
        self.opacity.clone()
    }
}
impl HasOpacity for Line {
    fn opacity_signal(&self) -> Signal<f32> {
        self.opacity.clone()
    }
}

fn show(n: &impl HasOpacity, d: Duration) -> Box<dyn Animation> {
    n.opacity_signal()
        .to(1.0, d)
        .ease(easings::cubic_out)
        .into()
}

fn hide(n: &impl HasOpacity, d: Duration) -> Box<dyn Animation> {
    n.opacity_signal().to(0.0, d).ease(easings::cubic_in).into()
}

// Helper for boxes to ensure they start completely hidden and slightly scaled down
fn create_box(label: &str, center: Vec2, size: Vec2, is_db: bool) -> (Rect, TextNode) {
    let top_left = Vec2::new(center.x - size.x / 2.0, center.y - size.y / 2.0);

    // approx text size: ~10.5px per char width, 20px height
    let text_width = label.len() as f32 * 10.5;
    let text_pos = Vec2::new(center.x - text_width / 2.0, center.y - 12.0);

    let r = Rect::default()
        .with_position(top_left)
        .with_size(size)
        .with_scale(0.95)
        .with_opacity(0.0)
        .with_radius(if is_db { 30.0 } else { 8.0 })
        .with_stroke(ACCENT, 2.0)
        .with_fill(Color::rgb8(0x26, 0x26, 0x2a));

    let t = TextNode::default()
        .with_position(text_pos)
        .with_text(label)
        .with_font_size(18.0)
        .with_font("JetBrains Mono")
        .with_fill(WHITE)
        .with_scale(0.95)
        .with_opacity(0.0);

    (r, t)
}

fn create_line(start: Vec2, end: Vec2) -> (Line, Vec2) {
    (
        Line::default()
            .with_start(start)
            .with_end(start) // will animate to 'end'
            .with_stroke(DIM, 2.0)
            .with_opacity(0.0),
        end,
    )
}

fn main() {
    let mut project = Project::default()
        .with_fps(60)
        .with_background(BG)
        .with_title("News Feed System")
        .with_dimensions(1920, 1080)
        .close_on_finish();

    // -- Title --
    let title = TextNode::default()
        .with_position(Vec2::new(80.0, 50.0))
        .with_text("News Feed Architecture")
        .with_font_size(52.0)
        .with_font("JetBrains Mono")
        .with_fill(WHITE)
        .with_scale(0.95)
        .with_opacity(0.0);

    // -- Top Tier --
    let (user_box, user_label) = create_box(
        "User (Web/App)",
        Vec2::new(960.0, 130.0),
        Vec2::new(200.0, 60.0),
        false,
    );

    let (line_lb, lb_end) = create_line(Vec2::new(960.0, 130.0), Vec2::new(960.0, 260.0));
    let (lb_box, lb_label) = create_box(
        "Load Balancer",
        Vec2::new(960.0, 260.0),
        Vec2::new(160.0, 60.0),
        false,
    );

    let (line_ws, ws_end) = create_line(Vec2::new(960.0, 260.0), Vec2::new(960.0, 390.0));
    let (ws_box, ws_label) = create_box(
        "Web Servers",
        Vec2::new(960.0, 390.0),
        Vec2::new(300.0, 80.0),
        false,
    );

    // -- Services Tier --
    let (line_post, post_end) = create_line(Vec2::new(960.0, 390.0), Vec2::new(600.0, 520.0));
    let (post_box, post_label) = create_box(
        "Post Service",
        Vec2::new(600.0, 520.0),
        Vec2::new(160.0, 60.0),
        false,
    );

    let (line_fanout, fanout_end) = create_line(Vec2::new(960.0, 390.0), Vec2::new(960.0, 520.0));
    let (fanout_box, fanout_label) = create_box(
        "Fanout Service",
        Vec2::new(960.0, 520.0),
        Vec2::new(180.0, 60.0),
        false,
    );

    let (line_notif, notif_end) = create_line(Vec2::new(960.0, 390.0), Vec2::new(1320.0, 520.0));
    let (notif_box, notif_label) = create_box(
        "Notification Service",
        Vec2::new(1320.0, 520.0),
        Vec2::new(240.0, 60.0),
        false,
    );

    // -- Post Caches & DB --
    let (line_post_cache, post_cache_end) =
        create_line(Vec2::new(600.0, 520.0), Vec2::new(450.0, 720.0));
    let (post_cache_box, post_cache_label) = create_box(
        "Post Cache",
        Vec2::new(450.0, 720.0),
        Vec2::new(160.0, 80.0),
        true,
    );

    let (line_post_db, post_db_end) = create_line(Vec2::new(600.0, 520.0), Vec2::new(750.0, 720.0));
    let (post_db_box, post_db_label) = create_box(
        "Post DB",
        Vec2::new(750.0, 720.0),
        Vec2::new(140.0, 80.0),
        true,
    );

    // -- NF Cache --
    let (line_nf_cache, nf_cache_end) =
        create_line(Vec2::new(960.0, 520.0), Vec2::new(960.0, 720.0));
    let (nf_cache_box, nf_cache_label) = create_box(
        "News Feed Cache",
        Vec2::new(960.0, 720.0),
        Vec2::new(200.0, 80.0),
        true,
    );

    // -- Fanout DBs Flow --
    let (line_graph_db, graph_db_end) =
        create_line(Vec2::new(960.0, 520.0), Vec2::new(1280.0, 620.0));
    let (graph_db_box, graph_db_label) = create_box(
        "Graph DB",
        Vec2::new(1280.0, 620.0),
        Vec2::new(140.0, 60.0),
        true,
    );

    let (line_user_cache, user_cache_end) =
        create_line(Vec2::new(960.0, 520.0), Vec2::new(1200.0, 720.0));
    let (user_cache_box, user_cache_label) = create_box(
        "User Cache",
        Vec2::new(1200.0, 720.0),
        Vec2::new(160.0, 80.0),
        true,
    );

    let (line_user_db, user_db_end) =
        create_line(Vec2::new(1200.0, 720.0), Vec2::new(1200.0, 860.0));
    let (user_db_box, user_db_label) = create_box(
        "User DB",
        Vec2::new(1200.0, 860.0),
        Vec2::new(140.0, 80.0),
        true,
    );

    // -- Case Indicators --
    let case1 = TextNode::default()
        .with_position(Vec2::new(80.0, 110.0))
        .with_text("Scenario 1: Cache Miss (Data Fallback)")
        .with_font_size(24.0)
        .with_font("JetBrains Mono")
        .with_fill(YELLOW)
        .with_opacity(0.0);

    let case2 = TextNode::default()
        .with_position(Vec2::new(80.0, 110.0))
        .with_text("Scenario 2: Cache Hit (Fast Path)")
        .with_font_size(24.0)
        .with_font("JetBrains Mono")
        .with_fill(GREEN)
        .with_opacity(0.0);

    let case3 = TextNode::default()
        .with_position(Vec2::new(80.0, 110.0))
        .with_text("Scenario 3: Rate Limited (Packet Dropped)")
        .with_font_size(24.0)
        .with_font("JetBrains Mono")
        .with_fill(RED)
        .with_opacity(0.0);

    // -- Flow Step Indicators --

    let case4 = TextNode::default()
        .with_position(Vec2::new(80.0, 110.0))
        .with_text("Scenario 4: Full Publish (Parallel)")
        .with_font_size(24.0)
        .with_font("JetBrains Mono")
        .with_fill(GREEN)
        .with_opacity(0.0);

    let step1 = TextNode::default()
        .with_position(Vec2::new(1120.0, 570.0))
        .with_text("1: Get Friend IDs")
        .with_font_size(16.0)
        .with_font("JetBrains Mono")
        .with_fill(YELLOW)
        .with_opacity(0.0);

    let step2 = TextNode::default()
        .with_position(Vec2::new(1000.0, 620.0))
        .with_text("2: Get Friend Data")
        .with_font_size(16.0)
        .with_font("JetBrains Mono")
        .with_fill(YELLOW)
        .with_opacity(0.0);

    let step3 = TextNode::default()
        .with_position(Vec2::new(760.0, 620.0))
        .with_text("3: Update Feed")
        .with_font_size(16.0)
        .with_font("JetBrains Mono")
        .with_fill(YELLOW)
        .with_opacity(0.0);

    // Initial packet using exact center point
    let packet = Circle::default()
        .with_position(Vec2::new(960.0, 130.0))
        .with_radius(12.0)
        .with_scale(0.95)
        .with_opacity(0.0)
        .with_fill(GREEN);

    let packet_post = Circle::default()
        .with_position(Vec2::new(960.0, 130.0))
        .with_radius(12.0)
        .with_scale(0.95)
        .with_opacity(0.0)
        .with_fill(ACCENT);

    let packet_notif = Circle::default()
        .with_position(Vec2::new(960.0, 130.0))
        .with_radius(12.0)
        .with_scale(0.95)
        .with_opacity(0.0)
        .with_fill(YELLOW);

    let fast_in = easings::cubic_out;
    let smooth = easings::cubic_in_out;
    let appear_dur = Duration::from_millis(250);
    let move_dur = Duration::from_millis(700);

    // -- Sequence --
    project.scene.video_timeline.add(chain![
        wait(Duration::from_millis(500)),
        all![
            show(&title, appear_dur),
            title
                .scale
                .to(Vec2::new(1.0, 1.0), appear_dur)
                .ease(fast_in)
        ],
        // Topology Staggered
        sequence![
            Duration::from_millis(60),
            all![
                show(&user_box, appear_dur),
                user_box
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
                show(&user_label, appear_dur),
                user_label
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
            ],
            all![
                show(&line_lb, appear_dur),
                line_lb.end.to(lb_end, appear_dur).ease(smooth),
                show(&lb_box, appear_dur),
                lb_box
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
                show(&lb_label, appear_dur),
                lb_label
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
            ],
            all![
                show(&line_ws, appear_dur),
                line_ws.end.to(ws_end, appear_dur).ease(smooth),
                show(&ws_box, appear_dur),
                ws_box
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
                show(&ws_label, appear_dur),
                ws_label
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
            ]
        ],
        sequence![
            Duration::from_millis(60),
            all![
                show(&line_post, appear_dur),
                line_post.end.to(post_end, appear_dur).ease(smooth),
                show(&post_box, appear_dur),
                post_box
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
                show(&post_label, appear_dur),
                post_label
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
            ],
            all![
                show(&line_fanout, appear_dur),
                line_fanout.end.to(fanout_end, appear_dur).ease(smooth),
                show(&fanout_box, appear_dur),
                fanout_box
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
                show(&fanout_label, appear_dur),
                fanout_label
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
            ],
            all![
                show(&line_notif, appear_dur),
                line_notif.end.to(notif_end, appear_dur).ease(smooth),
                show(&notif_box, appear_dur),
                notif_box
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
                show(&notif_label, appear_dur),
                notif_label
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
            ],
        ],
        sequence![
            Duration::from_millis(60),
            all![
                show(&line_post_cache, appear_dur),
                line_post_cache
                    .end
                    .to(post_cache_end, appear_dur)
                    .ease(smooth),
                show(&post_cache_box, appear_dur),
                post_cache_box
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
                show(&post_cache_label, appear_dur),
                post_cache_label
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
            ],
            all![
                show(&line_post_db, appear_dur),
                line_post_db.end.to(post_db_end, appear_dur).ease(smooth),
                show(&post_db_box, appear_dur),
                post_db_box
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
                show(&post_db_label, appear_dur),
                post_db_label
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
            ],
            all![
                show(&line_nf_cache, appear_dur),
                line_nf_cache.end.to(nf_cache_end, appear_dur).ease(smooth),
                show(&nf_cache_box, appear_dur),
                nf_cache_box
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
                show(&nf_cache_label, appear_dur),
                nf_cache_label
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
            ],
            all![
                show(&line_graph_db, appear_dur),
                line_graph_db.end.to(graph_db_end, appear_dur).ease(smooth),
                show(&graph_db_box, appear_dur),
                graph_db_box
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
                show(&graph_db_label, appear_dur),
                graph_db_label
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
            ],
            all![
                show(&line_user_cache, appear_dur),
                line_user_cache
                    .end
                    .to(user_cache_end, appear_dur)
                    .ease(smooth),
                show(&user_cache_box, appear_dur),
                user_cache_box
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
                show(&user_cache_label, appear_dur),
                user_cache_label
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
            ],
            all![
                show(&line_user_db, appear_dur),
                line_user_db.end.to(user_db_end, appear_dur).ease(smooth),
                show(&user_db_box, appear_dur),
                user_db_box
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
                show(&user_db_label, appear_dur),
                user_db_label
                    .scale
                    .to(Vec2::new(1.0, 1.0), appear_dur)
                    .ease(fast_in),
            ]
        ],
        wait(Duration::from_secs(1)),
        // ==========================================
        //  SCENARIO 1: CACHE MISS
        // ==========================================
        all![
            show(&case1, appear_dur),
            show(&packet, Duration::from_millis(200)),
            packet.fill_color.to(GREEN, Duration::from_millis(0)), // Start healthy
            packet
                .scale
                .to(Vec2::new(1.0, 1.0), Duration::from_millis(200))
                .ease(fast_in),
            packet
                .position
                .to(Vec2::new(960.0, 130.0), Duration::from_millis(0)),
        ],
        packet
            .position
            .to(Vec2::new(960.0, 260.0), move_dur)
            .ease(smooth),
        packet
            .position
            .to(Vec2::new(960.0, 390.0), move_dur)
            .ease(smooth),
        packet
            .position
            .to(Vec2::new(960.0, 520.0), move_dur)
            .ease(smooth),
        wait(Duration::from_millis(200)),
        // 1: Graph DB Check
        all![
            packet
                .position
                .to(Vec2::new(1280.0, 620.0), move_dur)
                .ease(smooth),
            delay![move_dur / 2, show(&step1, appear_dur)]
        ],
        packet
            .position
            .to(Vec2::new(960.0, 520.0), move_dur)
            .ease(smooth),
        // 2: User Cache MISSED -> Query DB
        all![
            packet
                .position
                .to(Vec2::new(1200.0, 720.0), move_dur)
                .ease(smooth),
            delay![move_dur / 2, show(&step2, appear_dur)]
        ],
        // Alert Miss: Flash Yellow and head to DB
        all![
            packet.fill_color.to(YELLOW, Duration::from_millis(200)),
            packet
                .scale
                .to(Vec2::new(1.2, 1.2), Duration::from_millis(200))
                .ease(easings::bounce_out),
            delay![
                Duration::from_millis(200),
                packet
                    .position
                    .to(Vec2::new(1200.0, 860.0), move_dur)
                    .ease(smooth)
            ]
        ],
        // Return from DB to Fanout smoothly
        all![
            packet.fill_color.to(GREEN, Duration::from_millis(200)),
            packet
                .scale
                .to(Vec2::new(1.0, 1.0), Duration::from_millis(200)),
            packet
                .position
                .to(Vec2::new(1200.0, 720.0), move_dur)
                .ease(smooth)
        ],
        packet
            .position
            .to(Vec2::new(960.0, 520.0), move_dur)
            .ease(smooth),
        // 3: Write to News Feed Cache
        all![
            packet
                .position
                .to(Vec2::new(960.0, 720.0), move_dur)
                .ease(smooth),
            delay![move_dur / 2, show(&step3, appear_dur)]
        ],
        packet
            .position
            .to(Vec2::new(960.0, 520.0), move_dur)
            .ease(smooth),
        wait(Duration::from_millis(200)),
        // Packet Flow (Back Track to User)
        packet
            .position
            .to(Vec2::new(960.0, 390.0), move_dur)
            .ease(smooth),
        packet
            .position
            .to(Vec2::new(960.0, 260.0), move_dur)
            .ease(smooth),
        packet
            .position
            .to(Vec2::new(960.0, 130.0), move_dur)
            .ease(smooth),
        // Hide packet & Case states
        all![
            hide(&packet, Duration::from_millis(200)),
            hide(&case1, appear_dur),
            hide(&step1, appear_dur),
            hide(&step2, appear_dur),
            hide(&step3, appear_dur),
        ],
        wait(Duration::from_secs(1)),
        // ==========================================
        //  SCENARIO 2: CACHE HIT
        // ==========================================
        all![
            show(&case2, appear_dur),
            show(&packet, Duration::from_millis(200)),
            packet
                .position
                .to(Vec2::new(960.0, 130.0), Duration::from_millis(0)),
            packet
                .scale
                .to(Vec2::new(1.0, 1.0), Duration::from_millis(200))
                .ease(fast_in),
        ],
        packet
            .position
            .to(Vec2::new(960.0, 260.0), move_dur)
            .ease(smooth),
        packet
            .position
            .to(Vec2::new(960.0, 390.0), move_dur)
            .ease(smooth),
        packet
            .position
            .to(Vec2::new(960.0, 520.0), move_dur)
            .ease(smooth),
        // 1: Graph DB
        all![
            packet
                .position
                .to(Vec2::new(1280.0, 620.0), move_dur)
                .ease(smooth),
            delay![move_dur / 2, show(&step1, appear_dur)]
        ],
        packet
            .position
            .to(Vec2::new(960.0, 520.0), move_dur)
            .ease(smooth),
        // 2: User Cache HIT (Skip DB entirely)
        all![
            packet
                .position
                .to(Vec2::new(1200.0, 720.0), move_dur)
                .ease(smooth),
            delay![move_dur / 2, show(&step2, appear_dur)]
        ],
        packet
            .position
            .to(Vec2::new(960.0, 520.0), move_dur)
            .ease(smooth),
        // 3: News Feed Cache
        all![
            packet
                .position
                .to(Vec2::new(960.0, 720.0), move_dur)
                .ease(smooth),
            delay![move_dur / 2, show(&step3, appear_dur)]
        ],
        packet
            .position
            .to(Vec2::new(960.0, 520.0), move_dur)
            .ease(smooth),
        // Return
        packet
            .position
            .to(Vec2::new(960.0, 390.0), move_dur)
            .ease(smooth),
        packet
            .position
            .to(Vec2::new(960.0, 260.0), move_dur)
            .ease(smooth),
        packet
            .position
            .to(Vec2::new(960.0, 130.0), move_dur)
            .ease(smooth),
        all![
            hide(&packet, Duration::from_millis(200)),
            hide(&case2, appear_dur),
            hide(&step1, appear_dur),
            hide(&step2, appear_dur),
            hide(&step3, appear_dur),
        ],
        wait(Duration::from_secs(1)),
        // ==========================================
        //  SCENARIO 3: DROPPED PACKET
        // ==========================================
        all![
            show(&case3, appear_dur),
            show(&packet, Duration::from_millis(200)),
            packet.fill_color.to(GREEN, Duration::from_millis(0)),
            packet
                .position
                .to(Vec2::new(960.0, 130.0), Duration::from_millis(0)),
            packet
                .scale
                .to(Vec2::new(1.0, 1.0), Duration::from_millis(200))
                .ease(fast_in),
        ],
        // To LB
        packet
            .position
            .to(Vec2::new(960.0, 260.0), move_dur)
            .ease(smooth),
        // Moving to Web Servers but fails mid-way / at LB
        all![
            packet
                .position
                .to(Vec2::new(960.0, 390.0), move_dur)
                .ease(smooth),
            // Turns Red and gets destroyed
            sequence![
                move_dur / 2, // Wait until halfway
                all![
                    packet.fill_color.to(RED, Duration::from_millis(150)),
                    packet
                        .scale
                        .to(Vec2::new(1.8, 1.8), Duration::from_millis(150))
                        .ease(easings::elastic_out),
                ],
                all![
                    hide(&packet, Duration::from_millis(200)),
                    packet
                        .scale
                        .to(Vec2::new(0.0, 0.0), Duration::from_millis(200))
                        .ease(easings::cubic_in),
                ]
            ]
        ],
        hide(&case3, appear_dur),
        wait(Duration::from_secs(1)),
        // ==========================================
        //  SCENARIO 4: PARALLEL PUBLISH
        // ==========================================
        all![
            show(&case4, appear_dur),
            show(&packet, Duration::from_millis(200)),
            show(&packet_post, Duration::from_millis(200)),
            show(&packet_notif, Duration::from_millis(200)),
            packet.fill_color.to(GREEN, Duration::from_millis(0)),
            packet
                .position
                .to(Vec2::new(960.0, 130.0), Duration::from_millis(0)),
            packet
                .scale
                .to(Vec2::new(1.0, 1.0), Duration::from_millis(200))
                .ease(fast_in),
            packet_post
                .position
                .to(Vec2::new(960.0, 130.0), Duration::from_millis(0)),
            packet_post
                .scale
                .to(Vec2::new(1.0, 1.0), Duration::from_millis(200))
                .ease(fast_in),
            packet_notif
                .position
                .to(Vec2::new(960.0, 130.0), Duration::from_millis(0)),
            packet_notif
                .scale
                .to(Vec2::new(1.0, 1.0), Duration::from_millis(200))
                .ease(fast_in),
        ],
        // Flow down collectively to Web Servers
        all![
            packet
                .position
                .to(Vec2::new(960.0, 260.0), move_dur)
                .ease(smooth),
            packet_post
                .position
                .to(Vec2::new(960.0, 260.0), move_dur)
                .ease(smooth),
            packet_notif
                .position
                .to(Vec2::new(960.0, 260.0), move_dur)
                .ease(smooth),
        ],
        all![
            packet
                .position
                .to(Vec2::new(960.0, 390.0), move_dur)
                .ease(smooth),
            packet_post
                .position
                .to(Vec2::new(960.0, 390.0), move_dur)
                .ease(smooth),
            packet_notif
                .position
                .to(Vec2::new(960.0, 390.0), move_dur)
                .ease(smooth),
        ],
        // BRANCHING!
        all![
            // Main packet goes to fanout
            packet
                .position
                .to(Vec2::new(960.0, 520.0), move_dur)
                .ease(smooth),
            // Post packet to Post Service
            packet_post
                .position
                .to(Vec2::new(600.0, 520.0), move_dur)
                .ease(smooth),
            // Notif packet to Notification Service
            packet_notif
                .position
                .to(Vec2::new(1320.0, 520.0), move_dur)
                .ease(smooth),
        ],
        wait(Duration::from_millis(200)),
        // Post DB, NF Cache, Notifications APNS fly off
        all![
            // Fanout updates NF Cache
            packet
                .position
                .to(Vec2::new(960.0, 720.0), move_dur)
                .ease(smooth),
            // Post updates DB
            packet_post
                .position
                .to(Vec2::new(750.0, 720.0), move_dur)
                .ease(smooth),
            // Notif flies off-screen (Apple Push Notification Server etc)
            packet_notif
                .position
                .to(Vec2::new(1500.0, 520.0), move_dur)
                .ease(smooth),
            hide(&packet_notif, move_dur),
        ],
        wait(Duration::from_millis(200)),
        // Return
        all![
            packet
                .position
                .to(Vec2::new(960.0, 520.0), move_dur)
                .ease(smooth),
            packet_post
                .position
                .to(Vec2::new(600.0, 520.0), move_dur)
                .ease(smooth),
        ],
        all![
            packet
                .position
                .to(Vec2::new(960.0, 390.0), move_dur)
                .ease(smooth),
            packet_post
                .position
                .to(Vec2::new(960.0, 390.0), move_dur)
                .ease(smooth),
        ],
        all![
            packet
                .position
                .to(Vec2::new(960.0, 260.0), move_dur)
                .ease(smooth),
            packet_post
                .position
                .to(Vec2::new(960.0, 260.0), move_dur)
                .ease(smooth),
        ],
        all![
            packet
                .position
                .to(Vec2::new(960.0, 130.0), move_dur)
                .ease(smooth),
            packet_post
                .position
                .to(Vec2::new(960.0, 130.0), move_dur)
                .ease(smooth),
        ],
        // Hide
        all![
            hide(&packet, Duration::from_millis(200)),
            hide(&packet_post, Duration::from_millis(200)),
            hide(&case4, appear_dur),
        ],
        wait(Duration::from_secs(2)),
    ]);

    // -- Adding to Scene --
    for element in [
        Box::new(title) as Box<dyn Node>,
        Box::new(line_lb),
        Box::new(line_ws),
        Box::new(line_post),
        Box::new(line_fanout),
        Box::new(line_notif),
        Box::new(line_post_cache),
        Box::new(line_post_db),
        Box::new(line_nf_cache),
        Box::new(line_graph_db),
        Box::new(line_user_cache),
        Box::new(line_user_db),
        Box::new(user_box),
        Box::new(user_label),
        Box::new(lb_box),
        Box::new(lb_label),
        Box::new(ws_box),
        Box::new(ws_label),
        Box::new(post_box),
        Box::new(post_label),
        Box::new(fanout_box),
        Box::new(fanout_label),
        Box::new(notif_box),
        Box::new(notif_label),
        Box::new(post_cache_box),
        Box::new(post_cache_label),
        Box::new(post_db_box),
        Box::new(post_db_label),
        Box::new(nf_cache_box),
        Box::new(nf_cache_label),
        Box::new(graph_db_box),
        Box::new(graph_db_label),
        Box::new(user_cache_box),
        Box::new(user_cache_label),
        Box::new(user_db_box),
        Box::new(user_db_label),
        Box::new(case1),
        Box::new(case2),
        Box::new(case3),
        Box::new(step1),
        Box::new(step2),
        Box::new(step3),
        Box::new(case4),
        Box::new(packet_post),
        Box::new(packet_notif),
        Box::new(packet),
    ] {
        project.scene.add(element);
    }

    project.show().expect("Failed to render");
}
