
//! A nice 2D graphics api.
//!
//! This attempt of designing a good type system for 2D graphics.

pub mod simple_pen;
pub mod advanced_pen;

/// Basic shapes.
pub enum Shape<'a, Fl=f64> {
    /// Draws a single pixel [x, y].
    Pixel(&'a [Fl, ..2]),
    /// Draws a horizontal line with one pixel width [x, y, dx].
    PixelHorizontalLine(&'a [Fl, ..3]),
    /// Draws a vertical line with one pixel width [x, y, dy].
    PixelVerticalLine(&'a [Fl, ..3]),
    /// Draws a line with one pixel in width.
    PixelLine(&'a [Fl, ..4]),
    /// Line [x1, y1, x2, y2].
    Line(&'a [Fl, ..4]),
    /// Rectangle [x, y, w, h].
    Rect(&'a [Fl, ..4]),
    /// Round rectangle [x, y, w, h, radius].
    RoundRect(&'a [Fl, ..5]),
    /// Ellipse [x, y, w, h].
    Ellipse(&'a [Fl, ..4]),
    /// Circle [x, y, radius].
    Circle(&'a [Fl, ..3]),
    /// Triangle [x1, y1, x2, y2, x3, y2].
    Triangle(&'a [Fl, ..6]),
    /// Closed polygon [x1, y1, ...].
    Polygon(&'a [Fl]),
    /// Catmull B-rom spline interpolated closed curve.
    /// [x1, y1, ...]
    Catmull(&'a [Fl]),
    /// Quadratic Bezier interpolated closed curve.
    /// Intersects the middle between each line.
    /// [x1, y1, x2, y2, ...].
    QuadraticBezierBound(&'a [Fl]),
    /// Quadratic Bezier interpolated with free tangents.
    /// [x1, y1, cx1, cy1, cx2, cy2, x2, y2, ...].
    QuadraticBezierFree(&'a [Fl]),
}

/// Describes the type of effect when drawing.
pub enum Effect<'a, Pen, Brush, Gradient> {
    /// Draw edges.
    Stroke(&'a Pen),
    /// Fill shape.
    Fill(&'a Brush),
    /// Gradient.
    Gradient(&'a Gradient),
}

pub type Matrix = [f64, ..6];

/// Renders a shape.
pub trait Shader {
    /// Draw shape.
    fn shade(&mut self, shape: &Shape);

    /// Gets a readonly view.
    fn get_view<'a>(&'a self) -> &'a View;

    /// Gets a mutable view.
    fn get_mut_view<'a>(&'a mut self) -> &'a mut View;
}

/// The transformation of graphics.
pub struct View {
    /// Base matrix transformation.
    pub base: [f64, ..6],
    /// Current matrix transformation.
    pub trans: [f64, ..6],
}


