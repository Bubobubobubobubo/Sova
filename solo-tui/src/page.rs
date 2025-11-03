// Map
// T C
// D S E
//   L V
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    #[default]
    Scene,
    Devices,
    Edit,
    Configure,
    Time,
    Logs,
    Vars,
}

impl Page {
    pub fn left(&mut self) {
        *self = match self {
            Page::Scene => Page::Devices,
            Page::Devices => Page::Devices,
            Page::Edit => Page::Scene,
            Page::Configure => Page::Time,
            Page::Time => Page::Time,
            Page::Logs => Page::Logs,
            Page::Vars => Page::Logs,
        }
    }

    pub fn right(&mut self) {
        *self = match self {
            Page::Scene => Page::Edit,
            Page::Devices => Page::Scene,
            Page::Edit => Page::Edit,
            Page::Configure => Page::Configure,
            Page::Time => Page::Configure,
            Page::Logs => Page::Vars,
            Page::Vars => Page::Vars,
        }
    }

    pub fn up(&mut self) {
        *self = match self {
            Page::Scene => Page::Configure,
            Page::Devices => Page::Time,
            Page::Edit => Page::Edit,
            Page::Configure => Page::Configure,
            Page::Time => Page::Time,
            Page::Logs => Page::Scene,
            Page::Vars => Page::Edit,
        }
    }

    pub fn down(&mut self) {
        *self = match self {
            Page::Scene => Page::Logs,
            Page::Devices => Page::Devices,
            Page::Edit => Page::Vars,
            Page::Configure => Page::Scene,
            Page::Time => Page::Devices,
            Page::Logs => Page::Logs,
            Page::Vars => Page::Vars,
        }
    }
}
