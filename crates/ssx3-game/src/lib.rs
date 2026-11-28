//! # ssx3-game
//!
//! Hand-written, typed Rust for SSX 3 functions, replacing the statically
//! recompiled versions one at a time (phase 2 in `docs/plans/roadmap.md`;
//! design in `docs/investigation/phase2-overrides.md`).
//!
//! A replacement is an ordinary `fn(&mut Cpu)` with the same contract as the
//! generated function: arguments and results in the MIPS registers
//! ([`abi`]), guest memory through typed views ([`guest`]), PS2 float
//! semantics ([`ps2f`]), MMI ops ([`mmi`]), calls to other guest functions
//! ([`call`]), event checks in loops ([`events`]), and the same EE cycle cost, so that runs stay
//! byte-identical. [`install`] puts them into the generated crates' slots.

// The generated EE code.
#[cfg(feature = "ee-threads")]
extern crate ssx3_ee_threads as ssx3_ee;

use ssx3_ee_rt::funcs::GuestFn;

pub mod abi;
#[cfg(feature = "ai")]
pub mod ai;
#[cfg(feature = "anim")]
pub mod anim;
#[cfg(feature = "audio")]
pub mod audio;
pub mod call;
#[cfg(feature = "camera")]
pub mod camera;
/// Debug-only overrides behind environment switches.
pub mod capture;
/// `SSX3_EVENT`: boot straight into an event (a development aid).
pub mod career_guard;
#[cfg(feature = "chase")]
pub mod chase;
#[cfg(feature = "containers")]
pub mod containers;
pub mod debug;
#[cfg(test)]
pub mod difftest;
#[cfg(feature = "dispatch")]
pub mod dispatch;
pub mod event_boot;
pub mod events;
/// The replacements, laid out like the C++ source tree (`src/<dir>/<unit>`).
pub mod game;
pub mod guest;
#[cfg(feature = "math")]
pub mod math;
pub mod mmi;
pub mod movie;
pub mod packetless;
pub mod ps2f;
#[cfg(feature = "render")]
pub mod render;
#[cfg(feature = "replay_script")]
pub mod replay_script;
pub mod rider;
#[cfg(feature = "rider_states")]
pub mod rider_states;
pub mod shadow;
#[cfg(feature = "skeleton")]
pub mod skeleton;
#[cfg(feature = "sound")]
pub mod sound;
#[cfg(feature = "state")]
pub mod state;
#[cfg(feature = "streaming")]
pub mod streaming;
#[cfg(feature = "ui")]
pub mod ui;
#[cfg(feature = "util")]
pub mod util;
#[cfg(feature = "world")]
pub mod world;
#[cfg(feature = "world_cache")]
pub mod world_cache;
// Phase 3: game files from the host (SSX3_NATIVE_FILES); uses `streaming`.
#[cfg(feature = "audio_system")]
pub mod audio_system;
#[cfg(feature = "graphics")]
pub mod graphics;
#[cfg(feature = "streaming")]
pub mod native_files;
#[cfg(feature = "oc643")]
pub mod oc643;
#[cfg(feature = "oc658")]
pub mod oc658;
pub mod plugin_host;
#[cfg(feature = "oc1460")]
pub mod oc1460;
#[cfg(feature = "oc1461")]
pub mod oc1461;
#[cfg(feature = "oc1462")]
pub mod oc1462;
#[cfg(feature = "oc1463")]
pub mod oc1463;
#[cfg(feature = "oc1464")]
pub mod oc1464;
#[cfg(feature = "oc1465")]
pub mod oc1465;
#[cfg(feature = "oc1466")]
pub mod oc1466;
#[cfg(feature = "oc1467")]
pub mod oc1467;
#[cfg(feature = "oc1468")]
pub mod oc1468;
#[cfg(feature = "oc1469")]
pub mod oc1469;
#[cfg(feature = "oc1470")]
pub mod oc1470;
#[cfg(feature = "oc1471")]
pub mod oc1471;
#[cfg(feature = "oc1472")]
pub mod oc1472;
#[cfg(feature = "oc1473")]
pub mod oc1473;
#[cfg(feature = "oc1474")]
pub mod oc1474;
#[cfg(feature = "oc1475")]
pub mod oc1475;
#[cfg(feature = "oc1476")]
pub mod oc1476;
#[cfg(feature = "oc1477")]
pub mod oc1477;
#[cfg(feature = "oc1478")]
pub mod oc1478;
#[cfg(feature = "oc1479")]
pub mod oc1479;
#[cfg(feature = "oc1480")]
pub mod oc1480;
#[cfg(feature = "oc1481")]
pub mod oc1481;
#[cfg(feature = "oc1482")]
pub mod oc1482;
#[cfg(feature = "oc1483")]
pub mod oc1483;
#[cfg(feature = "oc1484")]
pub mod oc1484;
#[cfg(feature = "oc1485")]
pub mod oc1485;
#[cfg(feature = "oc1486")]
pub mod oc1486;
#[cfg(feature = "oc1487")]
pub mod oc1487;
#[cfg(feature = "oc1488")]
pub mod oc1488;
#[cfg(feature = "oc1489")]
pub mod oc1489;
#[cfg(feature = "oc1490")]
pub mod oc1490;
#[cfg(feature = "oc1491")]
pub mod oc1491;
#[cfg(feature = "oc1492")]
pub mod oc1492;
#[cfg(feature = "oc1493")]
pub mod oc1493;
#[cfg(feature = "oc1494")]
pub mod oc1494;
#[cfg(feature = "oc1495")]
pub mod oc1495;
#[cfg(feature = "oc1496")]
pub mod oc1496;
#[cfg(feature = "oc1497")]
pub mod oc1497;
#[cfg(feature = "oc1498")]
pub mod oc1498;
#[cfg(feature = "oc1499")]
pub mod oc1499;
#[cfg(feature = "oc1500")]
pub mod oc1500;
#[cfg(feature = "oc1501")]
pub mod oc1501;
#[cfg(feature = "oc1502")]
pub mod oc1502;
#[cfg(feature = "oc1503")]
pub mod oc1503;
#[cfg(feature = "oc1504")]
pub mod oc1504;
#[cfg(feature = "oc1505")]
pub mod oc1505;
#[cfg(feature = "oc1506")]
pub mod oc1506;
#[cfg(feature = "oc1507")]
pub mod oc1507;
#[cfg(feature = "oc1508")]
pub mod oc1508;
#[cfg(feature = "oc1509")]
pub mod oc1509;
#[cfg(feature = "oc1510")]
pub mod oc1510;
#[cfg(feature = "oc1511")]
pub mod oc1511;
#[cfg(feature = "oc1512")]
pub mod oc1512;
#[cfg(feature = "oc1513")]
pub mod oc1513;
#[cfg(feature = "oc1514")]
pub mod oc1514;
#[cfg(feature = "oc1515")]
pub mod oc1515;
#[cfg(feature = "oc1516")]
pub mod oc1516;
#[cfg(feature = "oc1517")]
pub mod oc1517;
#[cfg(feature = "oc1518")]
pub mod oc1518;
#[cfg(feature = "oc1519")]
pub mod oc1519;
#[cfg(feature = "oc1520")]
pub mod oc1520;
#[cfg(feature = "oc1521")]
pub mod oc1521;
#[cfg(feature = "oc1522")]
pub mod oc1522;
#[cfg(feature = "oc1523")]
pub mod oc1523;
#[cfg(feature = "oc1524")]
pub mod oc1524;
#[cfg(feature = "oc1525")]
pub mod oc1525;
#[cfg(feature = "oc1526")]
pub mod oc1526;
#[cfg(feature = "oc1527")]
pub mod oc1527;
#[cfg(feature = "oc1528")]
pub mod oc1528;
#[cfg(feature = "oc1529")]
pub mod oc1529;
#[cfg(feature = "oc1530")]
pub mod oc1530;
#[cfg(feature = "oc1531")]
pub mod oc1531;
#[cfg(feature = "oc1532")]
pub mod oc1532;
#[cfg(feature = "oc1533")]
pub mod oc1533;
#[cfg(feature = "oc1534")]
pub mod oc1534;
#[cfg(feature = "oc1535")]
pub mod oc1535;
#[cfg(feature = "oc1536")]
pub mod oc1536;
#[cfg(feature = "oc1537")]
pub mod oc1537;
#[cfg(feature = "oc1538")]
pub mod oc1538;
#[cfg(feature = "oc1539")]
pub mod oc1539;
#[cfg(feature = "oc1540")]
pub mod oc1540;
#[cfg(feature = "oc1541")]
pub mod oc1541;
#[cfg(feature = "oc1542")]
pub mod oc1542;
#[cfg(feature = "oc1543")]
pub mod oc1543;
#[cfg(feature = "oc1544")]
pub mod oc1544;
#[cfg(feature = "oc1545")]
pub mod oc1545;
#[cfg(feature = "oc1546")]
pub mod oc1546;
#[cfg(feature = "oc1547")]
pub mod oc1547;
#[cfg(feature = "oc1548")]
pub mod oc1548;
#[cfg(feature = "oc1549")]
pub mod oc1549;
#[cfg(feature = "oc1550")]
pub mod oc1550;
#[cfg(feature = "oc1551")]
pub mod oc1551;
#[cfg(feature = "oc1552")]
pub mod oc1552;
#[cfg(feature = "oc1553")]
pub mod oc1553;
#[cfg(feature = "oc1554")]
pub mod oc1554;
#[cfg(feature = "oc1555")]
pub mod oc1555;
#[cfg(feature = "oc1556")]
pub mod oc1556;
#[cfg(feature = "oc1557")]
pub mod oc1557;
#[cfg(feature = "oc1558")]
pub mod oc1558;
#[cfg(feature = "oc1559")]
pub mod oc1559;
#[cfg(feature = "oc1560")]
pub mod oc1560;
#[cfg(feature = "oc1561")]
pub mod oc1561;
#[cfg(feature = "oc1562")]
pub mod oc1562;
#[cfg(feature = "oc1563")]
pub mod oc1563;
#[cfg(feature = "oc1564")]
pub mod oc1564;
#[cfg(feature = "oc1565")]
pub mod oc1565;
#[cfg(feature = "oc1566")]
pub mod oc1566;
#[cfg(feature = "oc1567")]
pub mod oc1567;
#[cfg(feature = "oc1568")]
pub mod oc1568;
#[cfg(feature = "oc1569")]
pub mod oc1569;
#[cfg(feature = "oc1570")]
pub mod oc1570;
#[cfg(feature = "oc1571")]
pub mod oc1571;
#[cfg(feature = "oc1572")]
pub mod oc1572;
#[cfg(feature = "oc1573")]
pub mod oc1573;
#[cfg(feature = "oc1574")]
pub mod oc1574;
#[cfg(feature = "oc1575")]
pub mod oc1575;
#[cfg(feature = "oc1576")]
pub mod oc1576;
#[cfg(feature = "oc1577")]
pub mod oc1577;
#[cfg(feature = "oc1578")]
pub mod oc1578;
#[cfg(feature = "oc1579")]
pub mod oc1579;
#[cfg(feature = "oc1580")]
pub mod oc1580;
#[cfg(feature = "oc1581")]
pub mod oc1581;
#[cfg(feature = "oc1582")]
pub mod oc1582;
#[cfg(feature = "oc1583")]
pub mod oc1583;
#[cfg(feature = "oc1584")]
pub mod oc1584;
#[cfg(feature = "oc1585")]
pub mod oc1585;
#[cfg(feature = "oc1586")]
pub mod oc1586;
#[cfg(feature = "oc1587")]
pub mod oc1587;
#[cfg(feature = "oc1588")]
pub mod oc1588;
#[cfg(feature = "oc1589")]
pub mod oc1589;
#[cfg(feature = "oc1590")]
pub mod oc1590;
#[cfg(feature = "oc1591")]
pub mod oc1591;
#[cfg(feature = "oc1592")]
pub mod oc1592;
#[cfg(feature = "oc1593")]
pub mod oc1593;
#[cfg(feature = "oc1594")]
pub mod oc1594;
#[cfg(feature = "oc1595")]
pub mod oc1595;
#[cfg(feature = "oc1596")]
pub mod oc1596;
#[cfg(feature = "oc1597")]
pub mod oc1597;
#[cfg(feature = "rider_logic")]
pub mod rider_logic;
#[cfg(feature = "snd_lib")]
pub mod snd_lib;
#[cfg(feature = "world_painter")]
pub mod world_painter;

/// The configurable tick rate.
pub mod tick;

/// Detail distance (terrain grids, rider LOD).

/// Replacements outside the work-package modules: (address, function, name).
const CORE: &[(u32, GuestFn, &str)] = &[(
    0x0011_e098,
    rider::build_world_matrix,
    "Rider_BuildWorldMatrix",
)];

/// Every installed replacement (core + enabled work packages). Each package
/// module exports its own `OVERRIDES`; all of them must be listed in
/// `overrides.txt`.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v: Vec<(u32, GuestFn, &'static str)> = CORE.to_vec();
    v.extend(game::overrides());
    #[cfg(feature = "oc1561")]
    v.extend_from_slice(oc1561::OVERRIDES);
    #[cfg(feature = "oc1562")]
    v.extend_from_slice(oc1562::OVERRIDES);
    #[cfg(feature = "oc1563")]
    v.extend_from_slice(oc1563::OVERRIDES);
    #[cfg(feature = "oc1564")]
    v.extend_from_slice(oc1564::OVERRIDES);
    #[cfg(feature = "oc1565")]
    v.extend_from_slice(oc1565::OVERRIDES);
    #[cfg(feature = "oc1566")]
    v.extend_from_slice(oc1566::OVERRIDES);
    #[cfg(feature = "oc1567")]
    v.extend_from_slice(oc1567::OVERRIDES);
    #[cfg(feature = "oc1568")]
    v.extend_from_slice(oc1568::OVERRIDES);
    #[cfg(feature = "oc1569")]
    v.extend_from_slice(oc1569::OVERRIDES);
    #[cfg(feature = "oc1570")]
    v.extend_from_slice(oc1570::OVERRIDES);
    #[cfg(feature = "oc1571")]
    v.extend_from_slice(oc1571::OVERRIDES);
    #[cfg(feature = "oc1572")]
    v.extend_from_slice(oc1572::OVERRIDES);
    #[cfg(feature = "oc1573")]
    v.extend_from_slice(oc1573::OVERRIDES);
    #[cfg(feature = "oc1574")]
    v.extend_from_slice(oc1574::OVERRIDES);
    #[cfg(feature = "oc1575")]
    v.extend_from_slice(oc1575::OVERRIDES);
    #[cfg(feature = "oc1576")]
    v.extend_from_slice(oc1576::OVERRIDES);
    #[cfg(feature = "oc1577")]
    v.extend_from_slice(oc1577::OVERRIDES);
    #[cfg(feature = "oc1578")]
    v.extend_from_slice(oc1578::OVERRIDES);
    #[cfg(feature = "oc1579")]
    v.extend_from_slice(oc1579::OVERRIDES);
    #[cfg(feature = "oc1580")]
    v.extend_from_slice(oc1580::OVERRIDES);
    #[cfg(feature = "oc1581")]
    v.extend_from_slice(oc1581::OVERRIDES);
    #[cfg(feature = "oc1582")]
    v.extend_from_slice(oc1582::OVERRIDES);
    #[cfg(feature = "oc1583")]
    v.extend_from_slice(oc1583::OVERRIDES);
    #[cfg(feature = "oc1584")]
    v.extend_from_slice(oc1584::OVERRIDES);
    #[cfg(feature = "oc1585")]
    v.extend_from_slice(oc1585::OVERRIDES);
    #[cfg(feature = "oc1586")]
    v.extend_from_slice(oc1586::OVERRIDES);
    #[cfg(feature = "oc1587")]
    v.extend_from_slice(oc1587::OVERRIDES);
    #[cfg(feature = "oc1588")]
    v.extend_from_slice(oc1588::OVERRIDES);
    #[cfg(feature = "oc1589")]
    v.extend_from_slice(oc1589::OVERRIDES);
    #[cfg(feature = "oc1590")]
    v.extend_from_slice(oc1590::OVERRIDES);
    #[cfg(feature = "oc1591")]
    v.extend_from_slice(oc1591::OVERRIDES);
    #[cfg(feature = "oc1592")]
    v.extend_from_slice(oc1592::OVERRIDES);
    #[cfg(feature = "oc1593")]
    v.extend_from_slice(oc1593::OVERRIDES);
    #[cfg(feature = "oc1594")]
    v.extend_from_slice(oc1594::OVERRIDES);
    #[cfg(feature = "oc1595")]
    v.extend_from_slice(oc1595::OVERRIDES);
    #[cfg(feature = "oc1596")]
    v.extend_from_slice(oc1596::OVERRIDES);
    #[cfg(feature = "oc1597")]
    v.extend_from_slice(oc1597::OVERRIDES);
    #[cfg(feature = "oc1521")]
    v.extend_from_slice(oc1521::OVERRIDES);
    #[cfg(feature = "oc1522")]
    v.extend_from_slice(oc1522::OVERRIDES);
    #[cfg(feature = "oc1523")]
    v.extend_from_slice(oc1523::OVERRIDES);
    #[cfg(feature = "oc1524")]
    v.extend_from_slice(oc1524::OVERRIDES);
    #[cfg(feature = "oc1525")]
    v.extend_from_slice(oc1525::OVERRIDES);
    #[cfg(feature = "oc1526")]
    v.extend_from_slice(oc1526::OVERRIDES);
    #[cfg(feature = "oc1527")]
    v.extend_from_slice(oc1527::OVERRIDES);
    #[cfg(feature = "oc1528")]
    v.extend_from_slice(oc1528::OVERRIDES);
    #[cfg(feature = "oc1529")]
    v.extend_from_slice(oc1529::OVERRIDES);
    #[cfg(feature = "oc1530")]
    v.extend_from_slice(oc1530::OVERRIDES);
    #[cfg(feature = "oc1531")]
    v.extend_from_slice(oc1531::OVERRIDES);
    #[cfg(feature = "oc1532")]
    v.extend_from_slice(oc1532::OVERRIDES);
    #[cfg(feature = "oc1533")]
    v.extend_from_slice(oc1533::OVERRIDES);
    #[cfg(feature = "oc1534")]
    v.extend_from_slice(oc1534::OVERRIDES);
    #[cfg(feature = "oc1535")]
    v.extend_from_slice(oc1535::OVERRIDES);
    #[cfg(feature = "oc1536")]
    v.extend_from_slice(oc1536::OVERRIDES);
    #[cfg(feature = "oc1537")]
    v.extend_from_slice(oc1537::OVERRIDES);
    #[cfg(feature = "oc1538")]
    v.extend_from_slice(oc1538::OVERRIDES);
    #[cfg(feature = "oc1539")]
    v.extend_from_slice(oc1539::OVERRIDES);
    #[cfg(feature = "oc1540")]
    v.extend_from_slice(oc1540::OVERRIDES);
    #[cfg(feature = "oc1541")]
    v.extend_from_slice(oc1541::OVERRIDES);
    #[cfg(feature = "oc1542")]
    v.extend_from_slice(oc1542::OVERRIDES);
    #[cfg(feature = "oc1543")]
    v.extend_from_slice(oc1543::OVERRIDES);
    #[cfg(feature = "oc1544")]
    v.extend_from_slice(oc1544::OVERRIDES);
    #[cfg(feature = "oc1545")]
    v.extend_from_slice(oc1545::OVERRIDES);
    #[cfg(feature = "oc1546")]
    v.extend_from_slice(oc1546::OVERRIDES);
    #[cfg(feature = "oc1547")]
    v.extend_from_slice(oc1547::OVERRIDES);
    #[cfg(feature = "oc1548")]
    v.extend_from_slice(oc1548::OVERRIDES);
    #[cfg(feature = "oc1549")]
    v.extend_from_slice(oc1549::OVERRIDES);
    #[cfg(feature = "oc1550")]
    v.extend_from_slice(oc1550::OVERRIDES);
    #[cfg(feature = "oc1551")]
    v.extend_from_slice(oc1551::OVERRIDES);
    #[cfg(feature = "oc1552")]
    v.extend_from_slice(oc1552::OVERRIDES);
    #[cfg(feature = "oc1553")]
    v.extend_from_slice(oc1553::OVERRIDES);
    #[cfg(feature = "oc1554")]
    v.extend_from_slice(oc1554::OVERRIDES);
    #[cfg(feature = "oc1555")]
    v.extend_from_slice(oc1555::OVERRIDES);
    #[cfg(feature = "oc1556")]
    v.extend_from_slice(oc1556::OVERRIDES);
    #[cfg(feature = "oc1557")]
    v.extend_from_slice(oc1557::OVERRIDES);
    #[cfg(feature = "oc1558")]
    v.extend_from_slice(oc1558::OVERRIDES);
    #[cfg(feature = "oc1559")]
    v.extend_from_slice(oc1559::OVERRIDES);
    #[cfg(feature = "oc1560")]
    v.extend_from_slice(oc1560::OVERRIDES);
    #[cfg(feature = "oc1520")]
    v.extend_from_slice(oc1520::OVERRIDES);
    #[cfg(feature = "oc1519")]
    v.extend_from_slice(oc1519::OVERRIDES);
    #[cfg(feature = "oc1518")]
    v.extend_from_slice(oc1518::OVERRIDES);
    #[cfg(feature = "oc1517")]
    v.extend_from_slice(oc1517::OVERRIDES);
    #[cfg(feature = "oc1516")]
    v.extend_from_slice(oc1516::OVERRIDES);
    #[cfg(feature = "oc1515")]
    v.extend_from_slice(oc1515::OVERRIDES);
    #[cfg(feature = "oc1514")]
    v.extend_from_slice(oc1514::OVERRIDES);
    #[cfg(feature = "oc1513")]
    v.extend_from_slice(oc1513::OVERRIDES);
    #[cfg(feature = "oc1512")]
    v.extend_from_slice(oc1512::OVERRIDES);
    #[cfg(feature = "oc1511")]
    v.extend_from_slice(oc1511::OVERRIDES);
    #[cfg(feature = "oc1510")]
    v.extend_from_slice(oc1510::OVERRIDES);
    #[cfg(feature = "oc1509")]
    v.extend_from_slice(oc1509::OVERRIDES);
    #[cfg(feature = "oc1508")]
    v.extend_from_slice(oc1508::OVERRIDES);
    #[cfg(feature = "oc1507")]
    v.extend_from_slice(oc1507::OVERRIDES);
    #[cfg(feature = "oc1506")]
    v.extend_from_slice(oc1506::OVERRIDES);
    #[cfg(feature = "oc1505")]
    v.extend_from_slice(oc1505::OVERRIDES);
    #[cfg(feature = "oc1504")]
    v.extend_from_slice(oc1504::OVERRIDES);
    #[cfg(feature = "oc1503")]
    v.extend_from_slice(oc1503::OVERRIDES);
    #[cfg(feature = "oc1502")]
    v.extend_from_slice(oc1502::OVERRIDES);
    #[cfg(feature = "oc1501")]
    v.extend_from_slice(oc1501::OVERRIDES);
    #[cfg(feature = "oc1489")]
    v.extend_from_slice(oc1489::OVERRIDES);
    #[cfg(feature = "oc1490")]
    v.extend_from_slice(oc1490::OVERRIDES);
    #[cfg(feature = "oc1491")]
    v.extend_from_slice(oc1491::OVERRIDES);
    #[cfg(feature = "oc1492")]
    v.extend_from_slice(oc1492::OVERRIDES);
    #[cfg(feature = "oc1493")]
    v.extend_from_slice(oc1493::OVERRIDES);
    #[cfg(feature = "oc1494")]
    v.extend_from_slice(oc1494::OVERRIDES);
    #[cfg(feature = "oc1495")]
    v.extend_from_slice(oc1495::OVERRIDES);
    #[cfg(feature = "oc1496")]
    v.extend_from_slice(oc1496::OVERRIDES);
    #[cfg(feature = "oc1497")]
    v.extend_from_slice(oc1497::OVERRIDES);
    #[cfg(feature = "oc1498")]
    v.extend_from_slice(oc1498::OVERRIDES);
    #[cfg(feature = "oc1499")]
    v.extend_from_slice(oc1499::OVERRIDES);
    #[cfg(feature = "oc1500")]
    v.extend_from_slice(oc1500::OVERRIDES);
    #[cfg(feature = "oc1460")]
    v.extend_from_slice(oc1460::OVERRIDES);
    #[cfg(feature = "oc1461")]
    v.extend_from_slice(oc1461::OVERRIDES);
    #[cfg(feature = "oc1462")]
    v.extend_from_slice(oc1462::OVERRIDES);
    #[cfg(feature = "oc1463")]
    v.extend_from_slice(oc1463::OVERRIDES);
    #[cfg(feature = "oc1464")]
    v.extend_from_slice(oc1464::OVERRIDES);
    #[cfg(feature = "oc1465")]
    v.extend_from_slice(oc1465::OVERRIDES);
    #[cfg(feature = "oc1466")]
    v.extend_from_slice(oc1466::OVERRIDES);
    #[cfg(feature = "oc1467")]
    v.extend_from_slice(oc1467::OVERRIDES);
    #[cfg(feature = "oc1468")]
    v.extend_from_slice(oc1468::OVERRIDES);
    #[cfg(feature = "oc1469")]
    v.extend_from_slice(oc1469::OVERRIDES);
    #[cfg(feature = "oc1470")]
    v.extend_from_slice(oc1470::OVERRIDES);
    #[cfg(feature = "oc1471")]
    v.extend_from_slice(oc1471::OVERRIDES);
    #[cfg(feature = "oc1472")]
    v.extend_from_slice(oc1472::OVERRIDES);
    #[cfg(feature = "oc1473")]
    v.extend_from_slice(oc1473::OVERRIDES);
    #[cfg(feature = "oc1474")]
    v.extend_from_slice(oc1474::OVERRIDES);
    #[cfg(feature = "oc1475")]
    v.extend_from_slice(oc1475::OVERRIDES);
    #[cfg(feature = "oc1476")]
    v.extend_from_slice(oc1476::OVERRIDES);
    #[cfg(feature = "oc1477")]
    v.extend_from_slice(oc1477::OVERRIDES);
    #[cfg(feature = "oc1478")]
    v.extend_from_slice(oc1478::OVERRIDES);
    #[cfg(feature = "oc1479")]
    v.extend_from_slice(oc1479::OVERRIDES);
    #[cfg(feature = "oc1480")]
    v.extend_from_slice(oc1480::OVERRIDES);
    #[cfg(feature = "oc1481")]
    v.extend_from_slice(oc1481::OVERRIDES);
    #[cfg(feature = "oc1482")]
    v.extend_from_slice(oc1482::OVERRIDES);
    #[cfg(feature = "oc1483")]
    v.extend_from_slice(oc1483::OVERRIDES);
    #[cfg(feature = "oc1484")]
    v.extend_from_slice(oc1484::OVERRIDES);
    #[cfg(feature = "oc1485")]
    v.extend_from_slice(oc1485::OVERRIDES);
    #[cfg(feature = "oc1486")]
    v.extend_from_slice(oc1486::OVERRIDES);
    #[cfg(feature = "oc1487")]
    v.extend_from_slice(oc1487::OVERRIDES);
    #[cfg(feature = "oc1488")]
    v.extend_from_slice(oc1488::OVERRIDES);
    #[cfg(feature = "oc658")]
    v.extend_from_slice(oc658::OVERRIDES);
    #[cfg(feature = "oc643")]
    v.extend_from_slice(oc643::OVERRIDES);
    v.extend_from_slice(movie::overrides());
    #[cfg(feature = "math")]
    v.extend_from_slice(math::OVERRIDES);
    #[cfg(feature = "anim")]
    v.extend_from_slice(anim::OVERRIDES);
    #[cfg(feature = "camera")]
    v.extend_from_slice(camera::OVERRIDES);
    #[cfg(feature = "render")]
    v.extend_from_slice(render::OVERRIDES);
    #[cfg(feature = "sound")]
    v.extend_from_slice(sound::OVERRIDES);
    #[cfg(feature = "state")]
    v.extend_from_slice(state::OVERRIDES);
    #[cfg(feature = "util")]
    v.extend_from_slice(util::OVERRIDES);
    #[cfg(feature = "dispatch")]
    v.extend_from_slice(dispatch::OVERRIDES);
    #[cfg(feature = "rider_states")]
    v.extend_from_slice(rider_states::OVERRIDES);
    #[cfg(feature = "skeleton")]
    v.extend_from_slice(skeleton::OVERRIDES);
    #[cfg(feature = "chase")]
    v.extend_from_slice(chase::OVERRIDES);
    #[cfg(feature = "ai")]
    v.extend_from_slice(ai::OVERRIDES);
    #[cfg(feature = "world")]
    v.extend_from_slice(world::OVERRIDES);
    #[cfg(feature = "ui")]
    v.extend_from_slice(ui::OVERRIDES);
    #[cfg(feature = "audio")]
    v.extend_from_slice(audio::OVERRIDES);
    #[cfg(feature = "containers")]
    v.extend_from_slice(containers::OVERRIDES);
    #[cfg(feature = "world_cache")]
    v.extend_from_slice(world_cache::OVERRIDES);
    #[cfg(feature = "replay_script")]
    v.extend_from_slice(replay_script::OVERRIDES);
    #[cfg(feature = "streaming")]
    v.extend_from_slice(streaming::OVERRIDES);
    #[cfg(feature = "graphics")]
    v.extend_from_slice(graphics::OVERRIDES);
    #[cfg(feature = "rider_logic")]
    v.extend_from_slice(rider_logic::OVERRIDES);
    #[cfg(feature = "world_painter")]
    v.extend_from_slice(world_painter::OVERRIDES);
    #[cfg(feature = "audio_system")]
    v.extend_from_slice(audio_system::OVERRIDES);
    #[cfg(feature = "snd_lib")]
    v.extend_from_slice(snd_lib::OVERRIDES);
    v
}

/// Threads the frame driver runs as steps (woken by a signal, run at the top of
/// the frame loop) instead of scheduling them by priority.
pub const STEP_THREAD_ENTRIES: &[u32] = &[
    0x003c_19a8, // SND_EeThread
    0x0031_ac60, // worker
];

/// The guest threads' entries and the waits (syscall return addresses) at which
/// each may unwind and be re-entered later (`docs/plans/single-loop.md`). Every
/// function between the entry and the wait is generated code (with a resume label
/// from `generated/entries.txt`) or a dispatcher-shaped port. A wait anywhere else
/// is done in place on the host stack.
pub const NATIVE_THREAD_ENTRIES: &[(u32, &[u32])] = &[
    (0x0010_0008, &[0x0031_aa8c]), // main thread (crt0 entry): PS2ExecMan_WaitFrame's wait
    (0x0031_ac08, &[0x0031_ac30]), // PS2ExecMan_VSyncThread
    (0x0031_ac60, &[0x0031_aca4]), // PS2ExecMan_WorkerThread: the job wait
    (0x003c_19a8, &[0x003c_19f0]), // SND_EeThread: the mixer wait
    (0x0042_4aa0, &[0x0042_4ae8]), // libkernl_IntrHelperThread (parks once)
    (0x003e_3be0, &[0x003e_3c20]), // EAFile_CdReadThread (parks once)
    (0x0040_09a8, &[0x0040_0a00]), // libcdvd_CallbackThread (parks once)
    (0x0038_2740, &[0x0038_27e0]), // Renderer_DmaKickThreadEntry (parks once)
];

/// Install the replacements (call once at boot, before guest code runs).
/// `SSX3_NO_OVERRIDES=1` keeps the generated bodies everywhere (A/B checks);
/// `SSX3_OVERRIDES=addr,addr` installs only the listed ones,
/// `SSX3_SKIP_OVERRIDES=addr,addr` all but the listed ones.
/// Returns how many were installed.
pub fn install() -> usize {
    if ssx3_core::config::var_os("SSX3_NO_OVERRIDES").is_some() {
        return 0;
    }
    let only: Option<Vec<u32>> = ssx3_core::config::var("SSX3_OVERRIDES").map(|s| {
        s.split(',')
            .filter_map(|a| u32::from_str_radix(a.trim().trim_start_matches("0x"), 16).ok())
            .collect()
    });
    // `SSX3_SKIP_OVERRIDES=addr,addr`: all but these (bisecting a wave).
    let skip: Vec<u32> = ssx3_core::config::var("SSX3_SKIP_OVERRIDES")
        .map(|s| {
            s.split(',')
                .filter_map(|a| u32::from_str_radix(a.trim().trim_start_matches("0x"), 16).ok())
                .collect()
        })
        .unwrap_or_default();
    let shadow = shadow::wanted();
    let mut n = 0;
    for (addr, f, name) in overrides() {
        if only.as_ref().is_some_and(|o| !o.contains(&addr)) || skip.contains(&addr) {
            continue;
        }
        // (`SSX3_SHADOW`: the replacement checked against the generated
        // body at run time, `shadow.rs`.)
        let f = match &shadow {
            Some(list) if list.as_ref().is_none_or(|l| l.contains(&addr)) => {
                shadow::wrap(addr, name, f).unwrap_or(f)
            }
            _ => f,
        };
        assert!(
            ssx3_ee::set_override(addr, f),
            "{name} ({addr:#x}) has no override slot: list it in crates/ssx3-game/overrides.txt and rerun recomp-ee"
        );
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    #[test]
    /// Every enabled replacement is listed in overrides.txt (the file may list
    /// more: packages not enabled in this build).
    fn overrides_txt_matches_table() {
        let txt: Vec<u32> = include_str!("../overrides.txt")
            .lines()
            .filter_map(|l| {
                let w = l.split('#').next()?.split_whitespace().next()?;
                u32::from_str_radix(w.trim_start_matches("0x"), 16).ok()
            })
            .collect();
        for (a, _, name) in super::overrides() {
            assert!(
                txt.contains(&a),
                "{name} ({a:#x}) is not listed in overrides.txt"
            );
        }
    }
}
