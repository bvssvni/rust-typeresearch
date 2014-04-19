
//! A more advanced pen suitable for diagrams. 

/// Describes how an end of a line should look like.
pub enum LineCap<Fl=f64> {
    /// Round edge.
    RoundLineCap,
    /// Arrow.
    ArrowLineCap,
    /// Square.
    SquareLineCap,
    /// Diamon (radius).
    DiamonLineCap(Fl),
    /// Circle (radius).
    CircleLineCap(Fl),
}

/// Describes how the edge between two lines should look like.
pub enum LineJoin<Fl=f64> {
    /// Round.
    RoundLineJoin,
    /// Square.
    SquareLineJoin,
    /// Diamon (radius).
    DiamonLineJoin(Fl),
    /// Circle (radius).
    CircleLineJoin(Fl),
}

/// A more advanced pen suitable for diagrams.
pub struct AdvancedPen<Fl=f64> {
    /// Border width from edge to edge.
    pub width: Fl,
    /// Pen color.
    pub color: [Fl, ..4],
    /// Line cap start.
    pub line_cap_start: LineCap<Fl>,
    /// Line cap end.
    pub line_cap_end: LineCap<Fl>,
    /// Line join.
    pub line_join: LineJoin<Fl>,
}

