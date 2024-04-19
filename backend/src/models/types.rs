use serde::{Deserialize, Serialize};
use diesel_derive_enum::DbEnum;
use macros::random_enum;
use rand::{distributions::{Distribution, Standard}, Rng};
use utoipa::ToSchema;
use strum_macros::{EnumIter, EnumString, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToSchema)]
#[derive(DbEnum, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[random_enum]
pub enum Gender {
    M,
    F,
    X
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, ToSchema)]
#[derive(DbEnum, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[random_enum]
pub enum UserType {
    Cliente,
    Agricultor,
    #[serde(alias = "Fabricante o Distribuidor de Agroinsumos", rename = "Fabricante o Distribuidor de Agroinsumos")]
    FabricanteODistribuidorDeAgroinsumos,
    #[serde(alias = "Proveedor de Seguros", rename = "Proveedor de Seguros")]
    ProveedorDeSeguros,
    Financiera,
    #[serde(alias = "Empresa CPG", rename = "Empresa CPG")]
    EmpresaCpg,
    Acopiador,
    Inversionista,
    #[serde(alias = "Publico en General", rename = "Publico en General")]
    PublicoEnGeneral
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize, EnumIter, EnumString, Display)]
pub enum RoleType {
    Admin,
    User
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToSchema)]
#[derive(Serialize, Deserialize, EnumIter, EnumString, Display)]
pub enum PermissionType {
    ViewDocuments,
    ViewDashboard,
    ViewDistribution,
    ViewUsers,
    ViewEdition,
    AddAccounts,
    EditAccounts,
    SendEmails,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(DbEnum, Serialize, Deserialize, EnumIter, EnumString, Display)]
pub enum EventType {
    Positive,
    Negative,
    Default
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(DbEnum, Serialize, Deserialize, EnumIter, EnumString, Display)]
pub enum FunctionType {
    Getter,
    Handler,
}

pub(crate) mod exports {
  pub use super::GenderMapping as Gender;
  pub use super::UserTypeMapping as UserType;
  pub use super::EventTypeMapping as EventType;
  pub use super::FunctionTypeMapping as FunctionType;
}
