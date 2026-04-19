use motion_canvas_rs::engine::easings;

#[test]
fn test_easings_boundaries() {
    let easings_list: Vec<fn(f32) -> f32> = vec![
        easings::back_in_out,
        easings::back_in,
        easings::back_out,
        easings::bounce_in_out,
        easings::bounce_in,
        easings::bounce_out,
        easings::circ_in_out,
        easings::circ_in,
        easings::circ_out,
        easings::cubic_in_out,
        easings::cubic_in,
        easings::cubic_out,
        easings::elastic_in_out,
        easings::elastic_in,
        easings::elastic_out,
        easings::expo_in_out,
        easings::expo_in,
        easings::expo_out,
        easings::linear,
        easings::quad_in_out,
        easings::quad_in,
        easings::quad_out,
        easings::quart_in_out,
        easings::quart_in,
        easings::quart_out,
        easings::quint_in_out,
        easings::quint_in,
        easings::quint_out,
        easings::sine_in_out,
        easings::sine_in,
        easings::sine_out,
    ];

    for easing in easings_list {
        assert!((easing(0.0) - 0.0).abs() < 1e-6, "Easing failed at 0.0");
        assert!((easing(1.0) - 1.0).abs() < 1e-6, "Easing failed at 1.0");
    }
}
