use motion_canvas_rs::engine::easings;

#[test]
fn test_easings_boundaries() {
    let easings_list: Vec<fn(f32) -> f32> = vec![
        easings::linear,
        easings::quad_in, easings::quad_out, easings::quad_in_out,
        easings::cubic_in, easings::cubic_out, easings::cubic_in_out,
        easings::sine_in, easings::sine_out, easings::sine_in_out,
        easings::elastic_in, easings::elastic_out,
    ];

    for easing in easings_list {
        assert!((easing(0.0) - 0.0).abs() < 1e-6, "Easing failed at 0.0");
        assert!((easing(1.0) - 1.0).abs() < 1e-6, "Easing failed at 1.0");
    }
}
