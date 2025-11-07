#[derive(Debug, Default, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum EsVersion {
    Es3,
    #[default]
    Es5,
    Es2015,
    Es2016,
    Es2017,
    Es2018,
    Es2019,
    Es2020,
    Es2021,
    Es2022,
    Es2023,
    Es2024,
    EsNext,
}
