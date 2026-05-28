use std::collections::HashMap;
use std::hash::Hash;

use super::Axis;

/// Scaler with a discrete domain and a discrete range.
///
/// ```
/// use vizkit::scale::ScaleOrdinal;
///
/// let scaler = ScaleOrdinal::default()
///     .domain(&["a", "b", "c"])
///     .range(&["red", "green", "blue"]);
///
/// for c in "abcdefgh".split("") {
///     match c {
///         "a" => assert_eq!(scaler.scale(c), Some("red").as_ref()),
///         "b" => assert_eq!(scaler.scale(c), Some("green").as_ref()),
///         "c" => assert_eq!(scaler.scale(c), Some("blue").as_ref()),
///         "d" => assert_eq!(scaler.scale(c), None),
///         "e" => assert_eq!(scaler.scale(c), None),
///         "f" => assert_eq!(scaler.scale(c), None),
///         "g" => assert_eq!(scaler.scale(c), None),
///         "h" => assert_eq!(scaler.scale(c), None),
///         "" => (),
///         x => unreachable!("char {} should not exist", x),
///     }
/// }
/// ```
#[derive(Default)]
pub struct ScaleOrdinal<D, R>
where
    D: Hash + Eq,
{
    index: HashMap<D, usize>,
    domain: Vec<D>,
    range: Vec<R>,
}

impl<D, R> ScaleOrdinal<D, R>
where
    D: Hash + Eq,
{
    /// Returns a new [`ScaleOrdinal`] with the specified domain applied.
    pub fn domain(self, domain: &[D]) -> Self
    where
        D: Clone,
    {
        let mut index = HashMap::new();
        let mut next_domain = Vec::new();
        for value in domain.iter() {
            if index.contains_key(value) {
                continue;
            }
            next_domain.push(value.clone());
            index.insert(value.clone(), next_domain.len() - 1);
        }
        Self {
            index,
            domain: next_domain,
            range: self.range,
        }
    }

    /// Returns a new [`ScaleOrdinal`] with the specified range applied.
    pub fn range(self, range: &[R]) -> Self
    where
        R: Clone,
    {
        Self {
            range: range.to_vec(),
            ..self
        }
    }

    /// Given the input, firstly it checks if the value exists in the domain, then it checks if it
    /// has a corresponding range value. If a value has been found, the value is returned. Otherwise
    /// it returns `None` (invalid value or empty range).
    pub fn scale(&self, x: D) -> Option<&R>
    where
        D: Clone,
    {
        match self.index.get(&x) {
            None => {
                // For non deterministic behavior, this code must be uncommented.
                // Howewer it implies to set `self` as `&mut self` which is not flexible.
                // self.domain.push(x.clone());
                // let i = self.domain.len() - 1;
                // self.index.insert(x.clone(), i);
                // if self.range.is_empty() {
                //     return None;
                // }
                // let index = i % self.range.len();
                // self.range.get(index)
                None
            }
            Some(i) => {
                if self.range.is_empty() {
                    return None;
                }
                let index = i % self.range.len();
                self.range.get(index)
            }
        }
    }
}

/// Scaler with a discrete domain and a continous range.
///
/// Additionally, it computes _band_ dimensions used for typically bar charts.
///
/// See [`ScaleDiscrete::step`] and [`ScaleDiscrete::bandwidth`].
///
///
/// ```
/// use vizkit::scale::ScaleDiscrete;
///
/// let scaler = ScaleDiscrete::band()
///     .domain(&["a", "b", "c"])
///     .range([0., 960.]);
///
/// for c in "abcd".split("") {
///     match c {
///         "a" => assert_eq!(scaler.scale(c), Some(0.)),
///         "b" => assert_eq!(scaler.scale(c), Some(320.)),
///         "c" => assert_eq!(scaler.scale(c), Some(640.)),
///         "d" => assert_eq!(scaler.scale(c), None),
///         "" => (),
///         x => unreachable!("char {} should not exist", x),
///     }
/// }
///
/// let scaler = ScaleDiscrete::point()
///     .domain(&["a", "b", "c"])
///     .range([0., 960.]);
///
/// for c in "abcd".split("") {
///     match c {
///         "a" => assert_eq!(scaler.scale(c), Some(0.)),
///         "b" => assert_eq!(scaler.scale(c), Some(480.)),
///         "c" => assert_eq!(scaler.scale(c), Some(960.)),
///         "d" => assert_eq!(scaler.scale(c), None),
///         "" => (),
///         x => unreachable!("char {} should not exist", x),
///     }
/// }
/// ```
pub struct ScaleDiscrete<D>
where
    D: Hash + Eq,
{
    r0: f32,
    r1: f32,
    step: f32,
    bandwidth: f32,
    padding_inner: f32,
    padding_outer: f32,
    align: f32,
    scale_ordinal: ScaleOrdinal<D, f32>,
}

impl<D> ScaleDiscrete<D>
where
    D: Hash + Eq,
{
    /// Returns a default [`ScaleDiscrete`] with uniform bands. You should use this default settings
    /// when you want ticks centered on the bandwidth.
    ///
    /// <svg height="60" viewBox="0 0 800 60" width="800" xmlns="http://www.w3.org/2000/svg">
    /// <g class="axis">
    /// <line stroke="currentColor" x1="74.28571" x2="74.28571" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(74.28571, 47.5)" y="0.71em">a</text>
    /// <line stroke="currentColor" x1="182.85715" x2="182.85715" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(182.85715, 47.5)" y="0.71em">b</text>
    /// <line stroke="currentColor" x1="291.42856" x2="291.42856" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(291.42856, 47.5)" y="0.71em">c</text>
    /// <line stroke="currentColor" x1="400" x2="400" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(400, 47.5)" y="0.71em">d</text>
    /// <line stroke="currentColor" x1="508.5714" x2="508.5714" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(508.5714, 47.5)" y="0.71em">e</text>
    /// <line stroke="currentColor" x1="617.1428" x2="617.1428" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(617.1428, 47.5)" y="0.71em">f</text>
    /// <line stroke="currentColor" x1="725.7143" x2="725.7143" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(725.7143, 47.5)" y="0.71em">g</text>
    /// </g>
    /// <g class="rects">
    /// <rect fill="none" height="36" stroke="currentColor" width="108.57143" x="20" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="108.57143" x="128.57143" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="108.57143" x="237.14285" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="108.57143" x="345.7143" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="108.57143" x="454.2857" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="108.57143" x="562.8571" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="108.57143" x="671.4286" y="2"/>
    /// </g>
    /// <rect fill="none" height="36" stroke="currentColor" stroke-opacity="0.2" width="760" x="20" y="2"/>
    /// </svg>
    pub fn band() -> Self
    where
        D: Default,
    {
        Self {
            r0: 0.,
            r1: 1.,
            step: 0.,
            bandwidth: 0.,
            padding_inner: 0.,
            padding_outer: 0.,
            align: 0.5,
            scale_ordinal: ScaleOrdinal::default(),
        }
        .rescale()
    }

    /// Returns a default [`ScaleDiscrete`] where the bandwidth is null.
    ///
    /// <svg height="60" viewBox="0 0 800 60" width="800" xmlns="http://www.w3.org/2000/svg">
    /// <g class="axis">
    /// <line stroke="currentColor" x1="20" x2="20" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(20, 47.5)" y="0.71em">a</text>
    /// <line stroke="currentColor" x1="146.66666" x2="146.66666" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(146.66666, 47.5)" y="0.71em">b</text>
    /// <line stroke="currentColor" x1="273.3333" x2="273.3333" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(273.3333, 47.5)" y="0.71em">c</text>
    /// <line stroke="currentColor" x1="400" x2="400" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(400, 47.5)" y="0.71em">d</text>
    /// <line stroke="currentColor" x1="526.6666" x2="526.6666" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(526.6666, 47.5)" y="0.71em">e</text>
    /// <line stroke="currentColor" x1="653.3333" x2="653.3333" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(653.3333, 47.5)" y="0.71em">f</text>
    /// <line stroke="currentColor" x1="780" x2="780" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(780, 47.5)" y="0.71em">g</text>
    /// </g>
    /// <g class="grid">
    /// <line stroke="currentColor" x1="20" x2="20" y1="2" y2="38"/>
    /// <line stroke="currentColor" x1="146.66666" x2="146.66666" y1="2" y2="38"/>
    /// <line stroke="currentColor" x1="273.3333" x2="273.3333" y1="2" y2="38"/>
    /// <line stroke="currentColor" x1="400" x2="400" y1="2" y2="38"/>
    /// <line stroke="currentColor" x1="526.6666" x2="526.6666" y1="2" y2="38"/>
    /// <line stroke="currentColor" x1="653.3333" x2="653.3333" y1="2" y2="38"/>
    /// <line stroke="currentColor" x1="780" x2="780" y1="2" y2="38"/>
    /// </g>
    /// <rect fill="none" height="36" stroke="currentColor" stroke-opacity="0.2" width="760" x="20" y="2"/>
    /// </svg>
    pub fn point() -> Self
    where
        D: Default,
    {
        Self {
            r0: 0.,
            r1: 1.,
            step: 0.,
            bandwidth: 0.,
            padding_inner: 1.,
            padding_outer: 0.,
            align: 0.5,
            scale_ordinal: ScaleOrdinal::default(),
        }
        .rescale()
    }

    /// Returns a new [`ScaleDiscrete`] with the specified domain applied.
    pub fn domain(self, domain: &[D]) -> Self
    where
        D: Clone,
    {
        Self {
            scale_ordinal: self.scale_ordinal.domain(domain),
            ..self
        }
        .rescale()
    }

    /// Returns a new [`ScaleDiscrete`] with the specified range applied.
    pub fn range(self, range: [f32; 2]) -> Self {
        let [r0, r1] = range;
        Self { r0, r1, ..self }.rescale()
    }

    /// Returns a new [`ScaleDiscrete`] in which inner and outer paddings are set to the same
    /// padding value. The inner and outer padding values range from 0.0 to 1.0.
    ///
    /// # Example
    ///
    /// padding: 0.4
    ///
    /// <svg height="60" viewBox="0 0 800 60" width="800" xmlns="http://www.w3.org/2000/svg">
    /// <g class="axis">
    /// <line stroke="currentColor" x1="91.89192" x2="91.89192" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(91.89192, 47.5)" y="0.71em">a</text>
    /// <line stroke="currentColor" x1="194.59462" x2="194.59462" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(194.59462, 47.5)" y="0.71em">b</text>
    /// <line stroke="currentColor" x1="297.29733" x2="297.29733" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(297.29733, 47.5)" y="0.71em">c</text>
    /// <line stroke="currentColor" x1="400.00003" x2="400.00003" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(400.00003, 47.5)" y="0.71em">d</text>
    /// <line stroke="currentColor" x1="502.70273" x2="502.70273" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(502.70273, 47.5)" y="0.71em">e</text>
    /// <line stroke="currentColor" x1="605.4054" x2="605.4054" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(605.4054, 47.5)" y="0.71em">f</text>
    /// <line stroke="currentColor" x1="708.1081" x2="708.1081" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(708.1081, 47.5)" y="0.71em">g</text>
    /// </g>
    /// <g class="rects">
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="61.081116" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="163.78381" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="266.4865" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="369.1892" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="471.8919" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="574.5946" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="677.2973" y="2"/>
    /// </g>
    /// <rect fill="none" height="36" stroke="currentColor" stroke-opacity="0.2" width="760" x="20" y="2"/>
    /// </svg>
    pub fn padding(self, padding: f32) -> Self {
        Self {
            padding_outer: padding.clamp(0., 1.),
            padding_inner: padding.clamp(0., 1.),
            ..self
        }
        .rescale()
    }

    /// Returns a new [`ScaleDiscrete`] in which inner padding is set to the padding value. The
    /// inner padding value ranges from 0.0 to 1.0;
    ///
    /// # Example
    ///
    /// padding_inner: 0.4
    ///
    /// <svg height="60" viewBox="0 0 800 60" width="800" xmlns="http://www.w3.org/2000/svg">
    /// <g class="axis">
    /// <line stroke="currentColor" x1="54.545456" x2="54.545456" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(54.545456, 47.5)" y="0.71em">a</text>
    /// <line stroke="currentColor" x1="169.69698" x2="169.69698" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(169.69698, 47.5)" y="0.71em">b</text>
    /// <line stroke="currentColor" x1="284.8485" x2="284.8485" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(284.8485, 47.5)" y="0.71em">c</text>
    /// <line stroke="currentColor" x1="400" x2="400" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(400, 47.5)" y="0.71em">d</text>
    /// <line stroke="currentColor" x1="515.15155" x2="515.15155" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(515.15155, 47.5)" y="0.71em">e</text>
    /// <line stroke="currentColor" x1="630.30304" x2="630.30304" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(630.30304, 47.5)" y="0.71em">f</text>
    /// <line stroke="currentColor" x1="745.4546" x2="745.4546" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(745.4546, 47.5)" y="0.71em">g</text>
    /// </g>
    /// <g class="rects">
    /// <rect fill="none" height="36" stroke="currentColor" width="69.09091" x="20" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="69.09091" x="135.15152" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="69.09091" x="250.30304" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="69.09091" x="365.45456" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="69.09091" x="480.60608" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="69.09091" x="595.75757" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="69.09091" x="710.9091" y="2"/>
    /// </g>
    /// <rect fill="none" height="36" stroke="currentColor" stroke-opacity="0.2" width="760" x="20" y="2"/>
    /// </svg>
    pub fn padding_inner(self, padding_inner: f32) -> Self {
        Self {
            padding_inner: padding_inner.clamp(0., 1.),
            ..self
        }
        .rescale()
    }

    /// Returns a new [`ScaleDiscrete`] with a new outer padding value. The outer padding value
    /// ranges from 0.0 to 1.0.
    ///
    /// # Example
    ///
    /// padding_outer: 0.4
    ///
    /// <svg height="60" viewBox="0 0 800 60" width="800" xmlns="http://www.w3.org/2000/svg">
    /// <g class="axis">
    /// <line stroke="currentColor" x1="107.692314" x2="107.692314" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(107.692314, 47.5)" y="0.71em">a</text>
    /// <line stroke="currentColor" x1="205.1282" x2="205.1282" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(205.1282, 47.5)" y="0.71em">b</text>
    /// <line stroke="currentColor" x1="302.56412" x2="302.56412" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(302.56412, 47.5)" y="0.71em">c</text>
    /// <line stroke="currentColor" x1="400" x2="400" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(400, 47.5)" y="0.71em">d</text>
    /// <line stroke="currentColor" x1="497.4359" x2="497.4359" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(497.4359, 47.5)" y="0.71em">e</text>
    /// <line stroke="currentColor" x1="594.8718" x2="594.8718" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(594.8718, 47.5)" y="0.71em">f</text>
    /// <line stroke="currentColor" x1="692.3077" x2="692.3077" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(692.3077, 47.5)" y="0.71em">g</text>
    /// </g>
    /// <g class="rects">
    /// <rect fill="none" height="36" stroke="currentColor" width="97.4359" x="58.974365" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="97.4359" x="156.41026" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="97.4359" x="253.84616" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="97.4359" x="351.28204" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="97.4359" x="448.71796" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="97.4359" x="546.1539" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="97.4359" x="643.5897" y="2"/>
    /// </g>
    /// <rect fill="none" height="36" stroke="currentColor" stroke-opacity="0.2" width="760" x="20" y="2"/>
    /// </svg>
    pub fn padding_outer(self, padding_outer: f32) -> Self {
        Self {
            padding_outer: padding_outer.clamp(0., 1.),
            ..self
        }
        .rescale()
    }

    /// Returns a new [`ScaleDiscrete`] in which align is set to the specified value. The align
    /// value ranges from 0.0 to 1.0.
    ///
    /// # Example
    ///
    /// padding: 0.4; align: 0.1
    ///
    /// <svg height="60" viewBox="0 0 800 60" width="800" xmlns="http://www.w3.org/2000/svg">
    /// <g class="axis">
    /// <line stroke="currentColor" x1="59.027035" x2="59.027035" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(59.027035, 47.5)" y="0.71em">a</text>
    /// <line stroke="currentColor" x1="161.72972" x2="161.72972" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(161.72972, 47.5)" y="0.71em">b</text>
    /// <line stroke="currentColor" x1="264.43243" x2="264.43243" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(264.43243, 47.5)" y="0.71em">c</text>
    /// <line stroke="currentColor" x1="367.13513" x2="367.13513" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(367.13513, 47.5)" y="0.71em">d</text>
    /// <line stroke="currentColor" x1="469.83783" x2="469.83783" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(469.83783, 47.5)" y="0.71em">e</text>
    /// <line stroke="currentColor" x1="572.5405" x2="572.5405" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(572.5405, 47.5)" y="0.71em">f</text>
    /// <line stroke="currentColor" x1="675.2432" x2="675.2432" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(675.2432, 47.5)" y="0.71em">g</text>
    /// </g>
    /// <g class="rects">
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="28.216225" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="130.91891" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="233.62161" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="336.3243" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="439.027" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="541.72974" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="644.43243" y="2"/>
    /// </g>
    /// <rect fill="none" height="36" stroke="currentColor" stroke-opacity="0.2" width="760" x="20" y="2"/>
    /// </svg>
    ///
    /// padding: 0.4; align: 0.9
    ///
    /// <svg height="60" viewBox="0 0 800 60" width="800" xmlns="http://www.w3.org/2000/svg">
    /// <g class="axis">
    /// <line stroke="currentColor" x1="124.75682" x2="124.75682" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(124.75682, 47.5)" y="0.71em">a</text>
    /// <line stroke="currentColor" x1="227.45952" x2="227.45952" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(227.45952, 47.5)" y="0.71em">b</text>
    /// <line stroke="currentColor" x1="330.16223" x2="330.16223" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(330.16223, 47.5)" y="0.71em">c</text>
    /// <line stroke="currentColor" x1="432.86493" x2="432.86493" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(432.86493, 47.5)" y="0.71em">d</text>
    /// <line stroke="currentColor" x1="535.5676" x2="535.5676" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(535.5676, 47.5)" y="0.71em">e</text>
    /// <line stroke="currentColor" x1="638.27026" x2="638.27026" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(638.27026, 47.5)" y="0.71em">f</text>
    /// <line stroke="currentColor" x1="740.97296" x2="740.97296" y1="38" y2="45.5"/>
    /// <text fill="currentColor" font-size="12" text-anchor="middle" transform="translate(740.97296, 47.5)" y="0.71em">g</text>
    /// </g>
    /// <g class="rects">
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="93.94601" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="196.64871" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="299.3514" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="402.0541" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="504.7568" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="607.4595" y="2"/>
    /// <rect fill="none" height="36" stroke="currentColor" width="61.62162" x="710.1622" y="2"/>
    /// </g>
    /// <rect fill="none" height="36" stroke="currentColor" stroke-opacity="0.2" width="760" x="20" y="2"/>
    /// </svg>
    pub fn align(self, align: f32) -> Self {
        Self {
            align: align.clamp(0., 1.),
            ..self
        }
        .rescale()
    }

    fn rescale(self) -> Self {
        let n = self.scale_ordinal.domain.len();
        let reverse = self.r1 < self.r0;
        let start = if reverse { self.r1 } else { self.r0 };
        let stop = if reverse { self.r0 } else { self.r1 };
        let step =
            (stop - start) / 1_f32.max(n as f32 - self.padding_inner + self.padding_outer * 2.);
        let start = start + (stop - start - step * (n as f32 - self.padding_inner)) * self.align;
        let bandwidth = step * (1. - self.padding_inner);
        let mut range: Vec<f32> = (0..n).map(|i| start + step * i as f32).collect();
        if reverse {
            range.reverse();
        }
        Self {
            scale_ordinal: self.scale_ordinal.range(&range),
            step,
            bandwidth,
            ..self
        }
    }

    /// Given the input, firstly it checks if the value exists in the domain, then it checks if it
    /// has a corresponding range value. If a value has been found, the value is returned. Otherwise
    /// it returns `None` (invalid value or empty range).
    pub fn scale(&self, x: D) -> Option<f32> {
        self.scale_ordinal
            .index
            .get(&x)
            .and_then(|i| {
                if self.scale_ordinal.range.is_empty() {
                    return None;
                }
                let index = i % self.scale_ordinal.range.len();
                self.scale_ordinal.range.get(index)
            })
            .copied()
    }

    /// Returns the distance between two adjacent bands.
    pub fn step(&self) -> f32 {
        self.step
    }

    /// Returns the width of each band.
    pub fn bandwidth(&self) -> f32 {
        self.bandwidth
    }
}

impl<D> Axis for ScaleDiscrete<D>
where
    D: Hash + Eq + Clone,
{
    type Tick = D;

    fn ticks(&self, _: Option<usize>) -> Vec<Self::Tick> {
        self.scale_ordinal.domain.to_vec()
    }

    fn tick_position(&self, x: Self::Tick) -> f32 {
        self.scale(x).unwrap_or(0.0) + self.bandwidth() * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_scale_ordinal() {
        let s = ScaleOrdinal::default()
            .domain(&[0, 1])
            .range(&["foo", "bar"]);
        assert_eq!(s.scale(0), Some("foo").as_ref());
        assert_eq!(s.scale(1), Some("bar").as_ref());
        assert_eq!(s.scale(2), None);
        assert_eq!(s.scale(-1), None);

        let s = s.range(&["a", "b", "c"]);
        assert_eq!(s.scale(0), Some("a").as_ref());
        assert_eq!(s.scale(1), Some("b").as_ref());
        assert_eq!(s.scale(2), None);

        let s = s.domain(&[0, 1, 2]);
        assert_eq!(s.scale(2), Some("c").as_ref());

        let s = s.domain(&[0, 1]);
        assert_eq!(s.scale(2), None);
    }

    fn band() -> ScaleDiscrete<&'static str> {
        ScaleDiscrete::band()
    }

    fn point() -> ScaleDiscrete<&'static str> {
        ScaleDiscrete::point()
    }

    #[rstest]
    #[case(band, 480., 40., 30., [0., 40., 80.], [7.5, 45., 82.5])]
    #[case(point, 960., 0., 30., [0., 60., 120.], [7.5, 45., 82.5])]
    fn test_scale_discrete(
        #[case] init: impl Fn() -> ScaleDiscrete<&'static str>,
        #[case] bar_value: f32,
        #[case] bandwidth1: f32,
        #[case] bandwidth2: f32,
        #[case] domain1: [f32; 3],
        #[case] domain2: [f32; 3],
    ) {
        let s = init().range([0., 960.]);
        assert_eq!(s.scale("foo"), None);

        let s = init().domain(&["foo", "bar"]).range([0., 960.]);
        assert_eq!(s.scale("foo"), Some(0.));
        assert_eq!(s.scale("bar"), Some(bar_value));

        let s = init().domain(&["a", "b", "c"]).range([0., 120.]);
        let range: Vec<f32> = s.ticks(None).iter().map(|x| s.tick_position(x)).collect();
        let bandwidth = s.bandwidth();
        let expected: Vec<f32> = domain1.map(|x| x + bandwidth * 0.5).into_iter().collect();
        assert_eq!(range, expected);
        assert_eq!(s.bandwidth(), bandwidth1);

        let s = s.padding(0.2);
        let range: Vec<f32> = s.ticks(None).iter().map(|x| s.tick_position(x)).collect();
        let bandwidth = s.bandwidth();
        let expected: Vec<f32> = domain2.map(|x| x + bandwidth * 0.5).into_iter().collect();
        assert_eq!(range, expected);
        assert_eq!(s.bandwidth(), bandwidth2);
    }
}
