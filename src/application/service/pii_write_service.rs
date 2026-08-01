//! UU PDP consent-gated writes for PII entities (coherence council #2).
//!
//! Each `create_*` method: checks a valid DataConsent exists for the employee + data category
//! before delegating to the standard CRUD `GenericCrudService::create`. If no valid consent →
//! `ServiceError::Validation` with the UU PDP message. This is the enforcement layer — PII
//! writes (Identity/Tax/Bpjs/Family/BankAccount/Contact) are *blocked* without consent.

use std::sync::Arc;

use backbone_core::{ServiceError, ServiceResult};
use sqlx::PgPool;

use crate::domain::entity::*;
use crate::presentation::dto::*;
use crate::{
    EmployeeBankAccountService, EmployeeBpjsService, EmployeeContactService,
    EmployeeFamilyService, EmployeeIdentityService, EmployeeTaxService,
};

use super::consent_guard;

/// Consent-gated write service for the 6 PII entities.
///
/// Wire this in the composer (or the HTTP layer) instead of the raw GenericCrudService
/// for PII entity creates, so every PII write is consent-checked.
pub struct PiiWriteService {
    pool: PgPool,
    identity: Arc<EmployeeIdentityService>,
    tax: Arc<EmployeeTaxService>,
    bpjs: Arc<EmployeeBpjsService>,
    family: Arc<EmployeeFamilyService>,
    bank: Arc<EmployeeBankAccountService>,
    contact: Arc<EmployeeContactService>,
}

impl PiiWriteService {
    pub fn new(
        pool: PgPool,
        identity: Arc<EmployeeIdentityService>,
        tax: Arc<EmployeeTaxService>,
        bpjs: Arc<EmployeeBpjsService>,
        family: Arc<EmployeeFamilyService>,
        bank: Arc<EmployeeBankAccountService>,
        contact: Arc<EmployeeContactService>,
    ) -> Self {
        Self { pool, identity, tax, bpjs, family, bank, contact }
    }

    // --- Identity (KTP / passport) → category: identity ---

    pub async fn create_identity(&self, dto: CreateEmployeeIdentityDto) -> ServiceResult<EmployeeIdentity> {
        consent_guard::require_consent(&self.pool, dto.employee_id, &DataCategory::Identity)
            .await
            .map_err(ServiceError::Validation)?;
        self.identity.create(dto).await
    }

    // --- Tax (NPWP) → category: financial ---

    pub async fn create_tax(&self, dto: CreateEmployeeTaxDto) -> ServiceResult<EmployeeTax> {
        consent_guard::require_consent(&self.pool, dto.employee_id, &DataCategory::Financial)
            .await
            .map_err(ServiceError::Validation)?;
        self.tax.create(dto).await
    }

    // --- BPJS → category: financial ---

    pub async fn create_bpjs(&self, dto: CreateEmployeeBpjsDto) -> ServiceResult<EmployeeBpjs> {
        consent_guard::require_consent(&self.pool, dto.employee_id, &DataCategory::Financial)
            .await
            .map_err(ServiceError::Validation)?;
        self.bpjs.create(dto).await
    }

    // --- Family / dependents → category: family ---

    pub async fn create_family(&self, dto: CreateEmployeeFamilyDto) -> ServiceResult<EmployeeFamily> {
        consent_guard::require_consent(&self.pool, dto.employee_id, &DataCategory::Family)
            .await
            .map_err(ServiceError::Validation)?;
        self.family.create(dto).await
    }

    // --- Bank account → category: financial ---

    pub async fn create_bank_account(&self, dto: CreateEmployeeBankAccountDto) -> ServiceResult<EmployeeBankAccount> {
        consent_guard::require_consent(&self.pool, dto.employee_id, &DataCategory::Financial)
            .await
            .map_err(ServiceError::Validation)?;
        self.bank.create(dto).await
    }

    // --- Contact → category: contact ---

    pub async fn create_contact(&self, dto: CreateEmployeeContactDto) -> ServiceResult<EmployeeContact> {
        consent_guard::require_consent(&self.pool, dto.employee_id, &DataCategory::Contact)
            .await
            .map_err(ServiceError::Validation)?;
        self.contact.create(dto).await
    }
}
