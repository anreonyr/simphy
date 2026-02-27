use crate::camera::WorldCamera;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
pub struct WorldMousePosition {
    pub position: Option<Vec2>,
}

pub fn get_world_mouse_position(
    camera_query: Single<(&Camera, &GlobalTransform), With<WorldCamera>>,
    window: Single<&Window, With<PrimaryWindow>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = *camera_query;
    let cursor_position = window.cursor_position()?;
    let world_pos = camera
        .viewport_to_world_2d(camera_transform, cursor_position)
        .ok()?;
    Some(world_pos)
}

pub fn update_world_mouse_position(
    mut world_mouse: ResMut<WorldMousePosition>,
    camera_query: Single<(&Camera, &GlobalTransform), With<WorldCamera>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    world_mouse.position = get_world_mouse_position(camera_query, window);
}

/// 生成圆弧顶点序列，用于创建 Polyline 碰撞体
/// - radius: 圆弧半径
/// - start_angle: 起始角度（弧度）
/// - end_angle: 结束角度（弧度）
/// - segments: 顶点数量，越多越平滑
pub fn generate_arc_vertices(
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: usize,
) -> Vec<Vec2> {
    // 计算每个顶点之间的角度步长
    let angle_step = (end_angle - start_angle) / segments as f32;
    // 从起始角度到结束角度生成顶点
    (0..=segments)
        .map(|i| {
            // 当前角度
            let angle = start_angle + angle_step * i as f32;
            // 极坐标转笛卡尔坐标
            Vec2::new(angle.cos() * radius, angle.sin() * radius)
        })
        .collect()
}

/// 生成闭合圆顶点序列
/// - radius: 圆半径
/// - segments: 顶点数量
pub fn generate_circle_vertices(radius: f32, segments: usize) -> Vec<Vec2> {
    generate_arc_vertices(radius, 0.0, std::f32::consts::TAU, segments)
}

/// 生成椭圆顶点序列
/// - radius_x: X轴半径
/// - radius_y: Y轴半径
/// - segments: 顶点数量
pub fn generate_ellipse_vertices(radius_x: f32, radius_y: f32, segments: usize) -> Vec<Vec2> {
    let angle_step = std::f32::consts::TAU / segments as f32;
    (0..segments)
        .map(|i| {
            let angle = angle_step * i as f32;
            Vec2::new(angle.cos() * radius_x, angle.sin() * radius_y)
        })
        .collect()
}

/// 生成矩形顶点序列（顺时针）
/// - width: 宽度
/// - height: 高度
pub fn generate_rectangle_vertices(width: f32, height: f32) -> Vec<Vec2> {
    let half_w = width / 2.0;
    let half_h = height / 2.0;
    vec![
        Vec2::new(-half_w, -half_h),
        Vec2::new(half_w, -half_h),
        Vec2::new(half_w, half_h),
        Vec2::new(-half_w, half_h),
    ]
}

/// 生成圆角矩形顶点序列
/// - width: 宽度
/// - height: 高度
/// - corner_radius: 角半径
/// - corner_segments: 每个圆角的顶点数
pub fn generate_rounded_rect_vertices(
    width: f32,
    height: f32,
    corner_radius: f32,
    corner_segments: usize,
) -> Vec<Vec2> {
    let half_w = width / 2.0;
    let half_h = height / 2.0;
    // 限制圆角半径
    let r = corner_radius.min(half_w).min(half_h);

    let inner_w = half_w - r;
    let inner_h = half_h - r;

    let mut vertices = Vec::new();

    // 右下角圆弧
    let start = 0.0;
    let end = std::f32::consts::FRAC_PI_2;
    for i in 0..=corner_segments {
        let angle = start + (end - start) * i as f32 / corner_segments as f32;
        vertices.push(Vec2::new(
            inner_w + angle.cos() * r,
            inner_h + angle.sin() * r,
        ));
    }

    // 左下角圆弧
    let start = std::f32::consts::FRAC_PI_2;
    let end = std::f32::consts::PI;
    for i in 1..=corner_segments {
        let angle = start + (end - start) * i as f32 / corner_segments as f32;
        vertices.push(Vec2::new(
            -inner_w + angle.cos() * r,
            inner_h + angle.sin() * r,
        ));
    }

    // 左上角圆弧
    let start = std::f32::consts::PI;
    let end = std::f32::consts::PI + std::f32::consts::FRAC_PI_2;
    for i in 1..=corner_segments {
        let angle = start + (end - start) * i as f32 / corner_segments as f32;
        vertices.push(Vec2::new(
            -inner_w + angle.cos() * r,
            -inner_h + angle.sin() * r,
        ));
    }

    // 右上角圆弧
    let start = std::f32::consts::PI + std::f32::consts::FRAC_PI_2;
    let end = std::f32::consts::TAU;
    for i in 1..=corner_segments {
        let angle = start + (end - start) * i as f32 / corner_segments as f32;
        vertices.push(Vec2::new(
            inner_w + angle.cos() * r,
            -inner_h + angle.sin() * r,
        ));
    }

    vertices
}

/// 生成正多边形顶点序列
/// - sides: 边数（至少3）
/// - radius: 外接圆半径
pub fn generate_regular_polygon_vertices(sides: usize, radius: f32) -> Vec<Vec2> {
    if sides < 3 {
        return vec![];
    }
    let angle_step = std::f32::consts::TAU / sides as f32;
    // 从 -PI/2 开始（让第一个顶点在正上方）
    let start_angle = -std::f32::consts::FRAC_PI_2;
    (0..sides)
        .map(|i| {
            let angle = start_angle + angle_step * i as f32;
            Vec2::new(angle.cos() * radius, angle.sin() * radius)
        })
        .collect()
}

/// 生成星形顶点序列
/// - points: 星星角数（至少4，会生成 points/2 个凸起）
/// - outer_radius: 外半径
/// - inner_radius: 内半径
pub fn generate_star_vertices(points: usize, outer_radius: f32, inner_radius: f32) -> Vec<Vec2> {
    if points < 4 {
        return vec![];
    }
    let angle_step = std::f32::consts::TAU / points as f32;
    let start_angle = -std::f32::consts::FRAC_PI_2;

    let mut vertices = Vec::with_capacity(points);
    for i in 0..points {
        let angle = start_angle + angle_step * i as f32;
        let radius = if i % 2 == 0 {
            outer_radius
        } else {
            inner_radius
        };
        vertices.push(Vec2::new(angle.cos() * radius, angle.sin() * radius));
    }
    vertices
}

/// 生成线段顶点序列
/// - start: 起点
/// - end: 终点
/// - segments: 中间插值点数
pub fn generate_line_vertices(start: Vec2, end: Vec2, segments: usize) -> Vec<Vec2> {
    if segments == 0 {
        return vec![start, end];
    }
    let step = (end - start) / segments as f32;
    (0..=segments).map(|i| start + step * i as f32).collect()
}

/// 生成十字形顶点序列
/// - width: 宽度
/// - height: 高度
pub fn generate_cross_vertices(width: f32, height: f32) -> Vec<Vec2> {
    let half_w = width / 2.0;
    let half_h = height / 2.0;
    let thickness = width.min(height) / 4.0;
    let half_t = thickness / 2.0;

    vec![
        // 水平部分
        Vec2::new(-half_w, -half_t),
        Vec2::new(half_w, -half_t),
        Vec2::new(half_w, half_t),
        Vec2::new(-half_w, half_t),
        // 竖直部分
        Vec2::new(-half_t, half_t),
        Vec2::new(-half_t, half_h),
        Vec2::new(half_t, half_h),
        Vec2::new(half_t, half_t),
        Vec2::new(half_t, -half_t),
        Vec2::new(half_t, -half_h),
        Vec2::new(-half_t, -half_h),
        Vec2::new(-half_t, -half_t),
    ]
}
