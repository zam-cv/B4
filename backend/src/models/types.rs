use serde::{Deserialize, Serialize};
use diesel_derive_enum::DbEnum;
use macros::random_enum;
use rand::{distributions::{Distribution, Standard}, Rng};
use utoipa::ToSchema;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToSchema)]
#[derive(DbEnum, Serialize, Deserialize)]
#[random_enum]
pub enum Gender {
    M,
    F,
    X
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToSchema)]
#[derive(DbEnum, Serialize, Deserialize)]
#[random_enum]
pub enum UserType {
    Cliente,
    Agricultor,
    FabricanteODistribuidorDeAgroinsumos,
    ProverdorDeSeguros,
    Financiera,
    EmpresaCPG,
    Acopiador,
    Inversionista,
    PublicoEnGeneral
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(DbEnum, Serialize, Deserialize, EnumIter)]
pub enum RoleType {
    Admin,
    User
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(DbEnum, Serialize, Deserialize, EnumIter)]
pub enum PermissionType {
    ViewDocuments,
    ViewDashboard,
    ViewDistribution,
    AddAccounts,
    EditAccounts,
    SendEmails,
}

pub(crate) mod exports {
  pub use super::GenderMapping as Gender;
  pub use super::UserTypeMapping as UserType;
  pub use super::RoleTypeMapping as RoleType;
  pub use super::PermissionTypeMapping as PermissionType;
}
