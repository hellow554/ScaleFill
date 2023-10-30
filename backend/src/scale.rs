use std::{
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::{bail, ensure};
use futures_util::Stream;
use num::Rational32;
use pin_project::pin_project;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader, Lines};
use tokio_serial::{
    DataBits, Parity, SerialPortBuilderExt, SerialPortType, SerialStream, StopBits,
};

use super::Result;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Unit {
    Newton,
    Kilogramm,
    Gramm,
    Pounds,
    Pieces,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Measurement(pub Rational32, pub Unit);

pub fn create_serial() -> Result<SerialStream> {
    for port in tokio_serial::available_ports()? {
        dbg!(&port);
        let SerialPortType::UsbPort(usb) = &port.port_type else {
            continue;
        };

        dbg!(usb);
    }

    let serial = tokio_serial::new("/dev/pts/4", 9600)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .open_native_async()?;

    Ok(serial)
}

#[pin_project]
pub struct Scale<S>(#[pin] Lines<BufReader<S>>);

impl<S: AsyncRead> Scale<S> {
    pub fn new(s: S) -> Self {
        Self(BufReader::new(s).lines())
    }
}

impl<S: AsyncRead> Stream for Scale<S> {
    type Item = Result<Measurement>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let next_line = this.0.poll_next_line(cx);

        match next_line {
            Poll::Ready(Ok(Some(line))) => Poll::Ready(Some(parse_line(&line))),
            Poll::Ready(Ok(None)) => Poll::Ready(None),
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e.into()))),
            Poll::Pending => Poll::Pending,
        }
    }
}

fn parse_line(str: &str) -> Result<Measurement, anyhow::Error> {
    let mut denom = 1;
    let mut numer = if str.starts_with('-') {
        -1
    } else {
        ensure!(
            str.starts_with('+'),
            "line must either start with + or -, but was {str:?}",
        );
        1
    };

    ensure!(str.len() >= 12, "string is too short: {str:?}");

    if let Some(idx) = str.find('.') {
        ensure!(
            idx == 7 || idx == 8,
            "Idx should either be 7 or 8, but is {idx}: {str:?}"
        );
        denom = if idx == 7 { 100 } else { 10 };

        numer *= str[1..idx].trim().parse::<i32>()?;
        numer = numer * denom + str[idx + 1..10].trim().parse::<i32>()?;
    } else {
        numer = str[1..10].trim().parse()?;
    }

    let unit = match &str[10..12] {
        " N" => Unit::Newton,
        "kg" => Unit::Kilogramm,
        " g" => Unit::Gramm,
        "lb" => Unit::Pounds,
        _ => bail!("Can't parse unit {str:?}"),
    };

    Ok(Measurement(Rational32::new(numer, denom), unit))
}

#[cfg(test)]
mod tests {
    use futures_util::StreamExt;

    use super::*;

    #[tokio::test]
    async fn some_lines() {
        const INPUT: &str = "+     0.00kg
+     0.14kg
+     0.22kg
+     0.10kg
+     0.08kg
+     0.16kg
+     0.28kg
+     0.38kg
+     0.50kg
+     0.62kg
+     0.70kg
+     0.84kg
+     0.96kg
+     1.06kg
+     1.20kg
+     1.30kg
+     1.44kg
+     1.56kg
+     1.64kg
+     1.78kg
+     1.88kg
+     2.02kg
+     2.14kg
+     2.22kg
+     2.32kg
+     2.42kg
+     2.52kg
+     2.58kg
+     2.66kg
+     2.74kg
+     2.84kg
+     2.90kg
+     2.98kg
+     3.00kg
+     3.08kg
+     3.14kg
+     3.18kg
+     3.26kg
+     3.28kg
+     3.36kg
+     3.40kg
+     3.46kg
+     3.56kg
+     3.66kg
+     3.78kg
+     3.92kg
+     3.98kg
+     4.12kg
+     4.26kg
+     4.38kg
+     4.52kg
+     4.62kg
+     4.74kg
+     4.90kg
+     5.04kg
+     5.08kg
+     5.10kg
+     5.18kg
+     4.06kg
+     3.82kg
+     5.12kg
+     5.12kg
";

        let values = &[
            Rational32::new(0, 1),
            Rational32::new(14, 100),
            Rational32::new(22, 100),
            Rational32::new(10, 100),
            Rational32::new(8, 100),
            Rational32::new(16, 100),
            Rational32::new(28, 100),
            Rational32::new(38, 100),
            Rational32::new(50, 100),
            Rational32::new(62, 100),
            Rational32::new(70, 100),
            Rational32::new(84, 100),
            Rational32::new(96, 100),
            Rational32::new(106, 100),
            Rational32::new(120, 100),
            Rational32::new(130, 100),
            Rational32::new(144, 100),
            Rational32::new(156, 100),
            Rational32::new(164, 100),
            Rational32::new(178, 100),
            Rational32::new(188, 100),
            Rational32::new(202, 100),
            Rational32::new(214, 100),
            Rational32::new(222, 100),
            Rational32::new(232, 100),
            Rational32::new(242, 100),
            Rational32::new(252, 100),
            Rational32::new(258, 100),
            Rational32::new(266, 100),
            Rational32::new(274, 100),
            Rational32::new(284, 100),
            Rational32::new(290, 100),
            Rational32::new(298, 100),
            Rational32::new(300, 100),
            Rational32::new(308, 100),
            Rational32::new(314, 100),
            Rational32::new(318, 100),
            Rational32::new(326, 100),
            Rational32::new(328, 100),
            Rational32::new(336, 100),
            Rational32::new(340, 100),
            Rational32::new(346, 100),
            Rational32::new(356, 100),
            Rational32::new(366, 100),
            Rational32::new(378, 100),
            Rational32::new(392, 100),
            Rational32::new(398, 100),
            Rational32::new(412, 100),
            Rational32::new(426, 100),
            Rational32::new(438, 100),
            Rational32::new(452, 100),
            Rational32::new(462, 100),
            Rational32::new(474, 100),
            Rational32::new(490, 100),
            Rational32::new(504, 100),
            Rational32::new(508, 100),
            Rational32::new(510, 100),
            Rational32::new(518, 100),
            Rational32::new(406, 100),
            Rational32::new(382, 100),
            Rational32::new(512, 100),
            Rational32::new(512, 100),
        ];

        let mut scale = Scale::new(INPUT.as_bytes());
        let mut i = 0;
        while let Some(m) = scale.next().await {
            assert_eq!(m.unwrap(), Measurement(values[i], Unit::Kilogramm));

            i += 1;
        }
    }

    #[tokio::test]
    async fn test_units() {
        const INPUT: &str = "+     0.01kg
+     0.02lb
+      1.2 N
+        5 g
+     8.00kg
";
        let mut scale = Scale::new(INPUT.as_bytes());

        let values: &[Measurement] = &[
            Measurement(Rational32::new(1, 100), Unit::Kilogramm),
            Measurement(Rational32::new(2, 100), Unit::Pounds),
            Measurement(Rational32::new(12, 10), Unit::Newton),
            Measurement(Rational32::new(5, 1), Unit::Gramm),
            Measurement(Rational32::new(8, 1), Unit::Kilogramm),
        ];

        let mut i = 0;
        while let Some(m) = scale.next().await {
            assert_eq!(m.unwrap(), values[i], "@ {i}");
            i += 1;
        }
    }
}
