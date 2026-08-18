//! `EmployeeQueryService` impl for [`crate::EmployeeModule`].
//!
//! Hand-written (user-owned — see `metaphor.codegen.yaml`). The generated `exports/services.rs`
//! declares the `EmployeeQueryService` port trait but no impl is generated for it; this file is
//! that impl. It is the seam every other module consumes employee through.
//!
//! Split:
//! - the **standard lookups** (`get_*` / `*_exists`) delegate to the existing `GenericCrudService`
//!   aliases (already wired on the module) and map entity → public DTO.
//! - the **custom read-port** `employee_ptkp` delegates to [`EmployeeTaxRepository::ptkp_override_for`]
//!   (override wins) else to [`EmployeeFamilyRepository::family_counts`] (derive from dependents),
//!   both of which hold the hand-written SQL (4-layer rule: services orchestrate, repos hold SQL).
//!
//! Company scoping (ADR-0008) is NOT done here — the caller (HTTP composition root via
//! `with_request_scope`, or a job via `with_company_scope`) sets it; `find_by_id` and the repos'
//! `company_scope::fetch_*_scoped` both honour the task-local RLS fence.

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entity::{
    Bank, DataConsent, DataSubjectRequest, Employee, EmployeeBankAccount, EmployeeBpjs,
    EmployeeCertification, EmployeeContact, EmployeeEducation, EmployeeFamily, EmployeeIdentity,
    EmployeeTax, EmployeeWorkExperience, Employment, EmploymentHistory, PiiAccessLog, PtkpTier,
    Religion,
};
// `exports::services` and `exports::types` are both private modules; their items are re-exported at
// `crate::exports::` — import through that, not the private module paths. The glob brings the public
// DTO/Summary/Id newtypes + the `EmployeeQueryService` trait. The domain entity module ALSO defines
// same-named id newtypes, so the entity structs above are imported by name (not `domain::entity::*`)
// to avoid a collision with the EXPORT id newtypes the glob brings in.
use crate::exports::EmployeeQueryService;
use crate::exports::*;
use crate::EmployeeModule;

#[async_trait]
impl EmployeeQueryService for EmployeeModule {
    async fn get_bank(&self, id: BankId) -> Result<Option<BankDto>> {
        let entity = self.bank_service.find_by_id(&id.into_inner().to_string()).await?;
        Ok(entity.map(bank_to_dto).transpose()?)
    }

    async fn get_bank_summary(&self, id: BankId) -> Result<Option<BankSummary>> {
        let entity = self.bank_service.find_by_id(&id.into_inner().to_string()).await?;
        Ok(entity.map(|e| BankSummary { id: BankId(e.id), name: e.name }))
    }

    async fn bank_exists(&self, id: BankId) -> Result<bool> {
        Ok(self.bank_service.find_by_id(&id.into_inner().to_string()).await?.is_some())
    }

    async fn get_data_consent(&self, id: DataConsentId) -> Result<Option<DataConsentDto>> {
        let entity = self
            .data_consent_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(data_consent_to_dto).transpose()?)
    }

    async fn get_data_consent_summary(&self, id: DataConsentId) -> Result<Option<DataConsentSummary>> {
        let entity = self
            .data_consent_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| DataConsentSummary { id: DataConsentId(e.id) }))
    }

    async fn data_consent_exists(&self, id: DataConsentId) -> Result<bool> {
        Ok(self
            .data_consent_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_data_subject_request(&self, id: DataSubjectRequestId) -> Result<Option<DataSubjectRequestDto>> {
        let entity = self
            .data_subject_request_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(data_subject_request_to_dto).transpose()?)
    }

    async fn get_data_subject_request_summary(
        &self,
        id: DataSubjectRequestId,
    ) -> Result<Option<DataSubjectRequestSummary>> {
        let entity = self
            .data_subject_request_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| DataSubjectRequestSummary {
            id: DataSubjectRequestId(e.id),
            status: e.status,
        }))
    }

    async fn data_subject_request_exists(&self, id: DataSubjectRequestId) -> Result<bool> {
        Ok(self
            .data_subject_request_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_employee(&self, id: EmployeeId) -> Result<Option<EmployeeDto>> {
        let entity = self
            .employee_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(employee_to_dto).transpose()?)
    }

    async fn get_employee_summary(&self, id: EmployeeId) -> Result<Option<EmployeeSummary>> {
        let entity = self
            .employee_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| EmployeeSummary {
            id: EmployeeId(e.id),
            first_name: e.first_name,
            last_name: e.last_name,
            email: e.email,
        }))
    }

    async fn employee_exists(&self, id: EmployeeId) -> Result<bool> {
        Ok(self
            .employee_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_employee_bank_account(&self, id: EmployeeBankAccountId) -> Result<Option<EmployeeBankAccountDto>> {
        let entity = self
            .employee_bank_account_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(employee_bank_account_to_dto).transpose()?)
    }

    async fn get_employee_bank_account_summary(
        &self,
        id: EmployeeBankAccountId,
    ) -> Result<Option<EmployeeBankAccountSummary>> {
        let entity = self
            .employee_bank_account_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| EmployeeBankAccountSummary {
            id: EmployeeBankAccountId(e.id),
            account_name: e.account_name,
        }))
    }

    async fn employee_bank_account_exists(&self, id: EmployeeBankAccountId) -> Result<bool> {
        Ok(self
            .employee_bank_account_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_employee_bpjs(&self, id: EmployeeBpjsId) -> Result<Option<EmployeeBpjsDto>> {
        let entity = self
            .employee_bpjs_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(employee_bpjs_to_dto).transpose()?)
    }

    async fn get_employee_bpjs_summary(&self, id: EmployeeBpjsId) -> Result<Option<EmployeeBpjsSummary>> {
        let entity = self
            .employee_bpjs_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| EmployeeBpjsSummary { id: EmployeeBpjsId(e.id) }))
    }

    async fn employee_bpjs_exists(&self, id: EmployeeBpjsId) -> Result<bool> {
        Ok(self
            .employee_bpjs_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_employee_certification(&self, id: EmployeeCertificationId) -> Result<Option<EmployeeCertificationDto>> {
        let entity = self
            .employee_certification_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(employee_certification_to_dto).transpose()?)
    }

    async fn get_employee_certification_summary(
        &self,
        id: EmployeeCertificationId,
    ) -> Result<Option<EmployeeCertificationSummary>> {
        let entity = self
            .employee_certification_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| EmployeeCertificationSummary {
            id: EmployeeCertificationId(e.id),
            name: e.name,
        }))
    }

    async fn employee_certification_exists(&self, id: EmployeeCertificationId) -> Result<bool> {
        Ok(self
            .employee_certification_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_employee_contact(&self, id: EmployeeContactId) -> Result<Option<EmployeeContactDto>> {
        let entity = self
            .employee_contact_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(employee_contact_to_dto).transpose()?)
    }

    async fn get_employee_contact_summary(&self, id: EmployeeContactId) -> Result<Option<EmployeeContactSummary>> {
        let entity = self
            .employee_contact_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| EmployeeContactSummary {
            id: EmployeeContactId(e.id),
            name: e.name,
            email: e.email,
        }))
    }

    async fn employee_contact_exists(&self, id: EmployeeContactId) -> Result<bool> {
        Ok(self
            .employee_contact_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_employee_education(&self, id: EmployeeEducationId) -> Result<Option<EmployeeEducationDto>> {
        let entity = self
            .employee_education_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(employee_education_to_dto).transpose()?)
    }

    async fn get_employee_education_summary(
        &self,
        id: EmployeeEducationId,
    ) -> Result<Option<EmployeeEducationSummary>> {
        let entity = self
            .employee_education_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| EmployeeEducationSummary {
            id: EmployeeEducationId(e.id),
            institution_name: e.institution_name,
        }))
    }

    async fn employee_education_exists(&self, id: EmployeeEducationId) -> Result<bool> {
        Ok(self
            .employee_education_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_employee_family(&self, id: EmployeeFamilyId) -> Result<Option<EmployeeFamilyDto>> {
        let entity = self
            .employee_family_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(employee_family_to_dto).transpose()?)
    }

    async fn get_employee_family_summary(&self, id: EmployeeFamilyId) -> Result<Option<EmployeeFamilySummary>> {
        let entity = self
            .employee_family_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| EmployeeFamilySummary {
            id: EmployeeFamilyId(e.id),
            name: e.name,
        }))
    }

    async fn employee_family_exists(&self, id: EmployeeFamilyId) -> Result<bool> {
        Ok(self
            .employee_family_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_employee_identity(&self, id: EmployeeIdentityId) -> Result<Option<EmployeeIdentityDto>> {
        let entity = self
            .employee_identity_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(employee_identity_to_dto).transpose()?)
    }

    async fn get_employee_identity_summary(&self, id: EmployeeIdentityId) -> Result<Option<EmployeeIdentitySummary>> {
        let entity = self
            .employee_identity_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| EmployeeIdentitySummary { id: EmployeeIdentityId(e.id) }))
    }

    async fn employee_identity_exists(&self, id: EmployeeIdentityId) -> Result<bool> {
        Ok(self
            .employee_identity_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_employee_tax(&self, id: EmployeeTaxId) -> Result<Option<EmployeeTaxDto>> {
        let entity = self
            .employee_tax_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(employee_tax_to_dto).transpose()?)
    }

    async fn get_employee_tax_summary(&self, id: EmployeeTaxId) -> Result<Option<EmployeeTaxSummary>> {
        let entity = self
            .employee_tax_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| EmployeeTaxSummary { id: EmployeeTaxId(e.id) }))
    }

    async fn employee_tax_exists(&self, id: EmployeeTaxId) -> Result<bool> {
        Ok(self
            .employee_tax_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_employee_work_experience(&self, id: EmployeeWorkExperienceId) -> Result<Option<EmployeeWorkExperienceDto>> {
        let entity = self
            .employee_work_experience_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(employee_work_experience_to_dto).transpose()?)
    }

    async fn get_employee_work_experience_summary(
        &self,
        id: EmployeeWorkExperienceId,
    ) -> Result<Option<EmployeeWorkExperienceSummary>> {
        let entity = self
            .employee_work_experience_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| EmployeeWorkExperienceSummary {
            id: EmployeeWorkExperienceId(e.id),
            company_name: e.company_name,
        }))
    }

    async fn employee_work_experience_exists(&self, id: EmployeeWorkExperienceId) -> Result<bool> {
        Ok(self
            .employee_work_experience_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_employment(&self, id: EmploymentId) -> Result<Option<EmploymentDto>> {
        let entity = self
            .employment_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(employment_to_dto).transpose()?)
    }

    async fn get_employment_summary(&self, id: EmploymentId) -> Result<Option<EmploymentSummary>> {
        let entity = self
            .employment_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| EmploymentSummary {
            id: EmploymentId(e.id),
            status: e.status,
        }))
    }

    async fn employment_exists(&self, id: EmploymentId) -> Result<bool> {
        Ok(self
            .employment_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_employment_history(&self, id: EmploymentHistoryId) -> Result<Option<EmploymentHistoryDto>> {
        let entity = self
            .employment_history_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(employment_history_to_dto).transpose()?)
    }

    async fn get_employment_history_summary(
        &self,
        id: EmploymentHistoryId,
    ) -> Result<Option<EmploymentHistorySummary>> {
        let entity = self
            .employment_history_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| EmploymentHistorySummary { id: EmploymentHistoryId(e.id) }))
    }

    async fn employment_history_exists(&self, id: EmploymentHistoryId) -> Result<bool> {
        Ok(self
            .employment_history_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_pii_access_log(&self, id: PiiAccessLogId) -> Result<Option<PiiAccessLogDto>> {
        let entity = self
            .pii_access_log_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(pii_access_log_to_dto).transpose()?)
    }

    async fn get_pii_access_log_summary(&self, id: PiiAccessLogId) -> Result<Option<PiiAccessLogSummary>> {
        let entity = self
            .pii_access_log_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| PiiAccessLogSummary { id: PiiAccessLogId(e.id) }))
    }

    async fn pii_access_log_exists(&self, id: PiiAccessLogId) -> Result<bool> {
        Ok(self
            .pii_access_log_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn get_religion(&self, id: ReligionId) -> Result<Option<ReligionDto>> {
        let entity = self
            .religion_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(religion_to_dto).transpose()?)
    }

    async fn get_religion_summary(&self, id: ReligionId) -> Result<Option<ReligionSummary>> {
        let entity = self
            .religion_service
            .find_by_id(&id.into_inner().to_string())
            .await?;
        Ok(entity.map(|e| ReligionSummary {
            id: ReligionId(e.id),
            name: e.name,
        }))
    }

    async fn religion_exists(&self, id: ReligionId) -> Result<bool> {
        Ok(self
            .religion_service
            .find_by_id(&id.into_inner().to_string())
            .await?
            .is_some())
    }

    async fn employee_ptkp(&self, employee_id: Uuid) -> Result<PtkpTier> {
        // Override wins: an explicit ptkp_override short-circuits derivation.
        if let Some(tier) = self
            .employee_tax_repository
            .ptkp_override_for(&self.db_pool, employee_id)
            .await?
        {
            return Ok(tier);
        }
        // Derive: married = EXISTS spouse; dependents = min(child count, 3).
        let (spouse, children) = self
            .employee_family_repository
            .family_counts(&self.db_pool, employee_id)
            .await?;
        let dependents = children.min(3);
        Ok(if spouse > 0 {
            match dependents {
                0 => PtkpTier::K0,
                1 => PtkpTier::K1,
                2 => PtkpTier::K2,
                _ => PtkpTier::K3,
            }
        } else {
            match dependents {
                0 => PtkpTier::Tk0,
                1 => PtkpTier::Tk1,
                2 => PtkpTier::Tk2,
                _ => PtkpTier::Tk3,
            }
        })
    }

    async fn statutory_inputs(&self, employee_id: Uuid) -> Result<StatutoryInputs> {
        // One JOIN across taxes/bpjs/employments (anchored on the employee row). Missing rows degrade
        // to NULL columns rather than dropping the bundle; only a missing employee returns None.
        let row = self
            .employee_tax_repository
            .statutory_row_for(&self.db_pool, employee_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("employee not found: {}", employee_id))?;

        // PTKP reuses the existing port (override wins, else derived from dependents) — single source
        // of truth for the tier so the two reads can never disagree.
        let ptkp = self.employee_ptkp(employee_id).await?;

        // has_npwp = a non-empty npwp_number. An empty string counts as "no NPWP" so a blank upload
        // doesn't accidentally dodge the 20% surtax.
        let has_npwp = row
            .npwp_number
            .as_deref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);

        Ok(StatutoryInputs {
            ptkp,
            has_npwp,
            bpjs_kesehatan_family: row.bpjs_kesehatan_family,
            join_date: row.join_date,
            ter_category: row.ter_category,
        })
    }
}

// ─── entity → public DTO mapping ───────────────────────────────────────────────
//
// The only non-trivial conversion is `metadata`: the entity holds a typed `AuditMetadata`, the
// public DTO exposes it as an opaque `serde_json::Value` (so consumers don't depend on the internal
// audit struct's shape). `PiiAccessLogDto` carries no metadata, so its mapper omits it.

fn bank_to_dto(e: Bank) -> Result<BankDto> {
    Ok(BankDto {
        id: BankId(e.id),
        name: e.name,
        code: e.code,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn data_consent_to_dto(e: DataConsent) -> Result<DataConsentDto> {
    Ok(DataConsentDto {
        id: DataConsentId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        data_category: e.data_category,
        lawful_basis: e.lawful_basis,
        consent_given_at: e.consent_given_at,
        consent_method: e.consent_method,
        privacy_notice_version: e.privacy_notice_version,
        retention_until: e.retention_until,
        withdrawn_at: e.withdrawn_at,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn data_subject_request_to_dto(e: DataSubjectRequest) -> Result<DataSubjectRequestDto> {
    Ok(DataSubjectRequestDto {
        id: DataSubjectRequestId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        request_type: e.request_type,
        status: e.status,
        requested_at: e.requested_at,
        fulfilled_at: e.fulfilled_at,
        response: e.response,
        note: e.note,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn employee_to_dto(e: Employee) -> Result<EmployeeDto> {
    Ok(EmployeeDto {
        id: EmployeeId(e.id),
        company_id: e.company_id,
        employee_number: e.employee_number,
        user_id: e.user_id,
        first_name: e.first_name,
        last_name: e.last_name,
        email: e.email,
        mobile_phone: e.mobile_phone,
        phone: e.phone,
        birth_place: e.birth_place,
        birth_date: e.birth_date,
        gender: e.gender,
        marital_status: e.marital_status,
        blood_type: e.blood_type,
        religion_id: e.religion_id,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn employee_bank_account_to_dto(e: EmployeeBankAccount) -> Result<EmployeeBankAccountDto> {
    Ok(EmployeeBankAccountDto {
        id: EmployeeBankAccountId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        bank_id: e.bank_id,
        account_number: e.account_number,
        account_name: e.account_name,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn employee_bpjs_to_dto(e: EmployeeBpjs) -> Result<EmployeeBpjsDto> {
    Ok(EmployeeBpjsDto {
        id: EmployeeBpjsId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        bpjs_ketenagakerjaan_number: e.bpjs_ketenagakerjaan_number,
        npp_bpjs_ketenagakerjaan: e.npp_bpjs_ketenagakerjaan,
        bpjs_ketenagakerjaan_date: e.bpjs_ketenagakerjaan_date,
        bpjs_kesehatan_number: e.bpjs_kesehatan_number,
        bpjs_kesehatan_family: e.bpjs_kesehatan_family,
        bpjs_kesehatan_date: e.bpjs_kesehatan_date,
        jaminan_pensiun_date: e.jaminan_pensiun_date,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn employee_certification_to_dto(e: EmployeeCertification) -> Result<EmployeeCertificationDto> {
    Ok(EmployeeCertificationDto {
        id: EmployeeCertificationId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        name: e.name,
        issuing_organization: e.issuing_organization,
        start_date: e.start_date,
        end_date: e.end_date,
        description: e.description,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn employee_contact_to_dto(e: EmployeeContact) -> Result<EmployeeContactDto> {
    Ok(EmployeeContactDto {
        id: EmployeeContactId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        name: e.name,
        phone: e.phone,
        email: e.email,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn employee_education_to_dto(e: EmployeeEducation) -> Result<EmployeeEducationDto> {
    Ok(EmployeeEducationDto {
        id: EmployeeEducationId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        institution_name: e.institution_name,
        major: e.major,
        field: e.field,
        score: e.score,
        start_year: e.start_year,
        end_year: e.end_year,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn employee_family_to_dto(e: EmployeeFamily) -> Result<EmployeeFamilyDto> {
    Ok(EmployeeFamilyDto {
        id: EmployeeFamilyId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        name: e.name,
        relationship: e.relationship,
        birth_date: e.birth_date,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn employee_identity_to_dto(e: EmployeeIdentity) -> Result<EmployeeIdentityDto> {
    Ok(EmployeeIdentityDto {
        id: EmployeeIdentityId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        identity_type: e.identity_type,
        identity_number: e.identity_number,
        identity_expiry_date: e.identity_expiry_date,
        is_permanent: e.is_permanent,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn employee_tax_to_dto(e: EmployeeTax) -> Result<EmployeeTaxDto> {
    Ok(EmployeeTaxDto {
        id: EmployeeTaxId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        npwp_number: e.npwp_number,
        ptkp_override: e.ptkp_override,
        tax_method: e.tax_method,
        ter_category: e.ter_category,
        tax_salary: e.tax_salary,
        taxable_date: e.taxable_date,
        beginning_netto: e.beginning_netto,
        pph21_paid: e.pph21_paid,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn employee_work_experience_to_dto(e: EmployeeWorkExperience) -> Result<EmployeeWorkExperienceDto> {
    Ok(EmployeeWorkExperienceDto {
        id: EmployeeWorkExperienceId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        company_name: e.company_name,
        job_position: e.job_position,
        start_date: e.start_date,
        end_date: e.end_date,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn employment_to_dto(e: Employment) -> Result<EmploymentDto> {
    Ok(EmploymentDto {
        id: EmploymentId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        employment_status: e.employment_status,
        join_date: e.join_date,
        end_join_date: e.end_join_date,
        department_id: e.department_id,
        level_id: e.level_id,
        position_id: e.position_id,
        direct_manager_id: e.direct_manager_id,
        status: e.status,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn employment_history_to_dto(e: EmploymentHistory) -> Result<EmploymentHistoryDto> {
    Ok(EmploymentHistoryDto {
        id: EmploymentHistoryId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        effective_date: e.effective_date,
        action: e.action,
        position_id_from: e.position_id_from,
        position_id_to: e.position_id_to,
        level_id_from: e.level_id_from,
        level_id_to: e.level_id_to,
        department_id_from: e.department_id_from,
        department_id_to: e.department_id_to,
        reference_id: e.reference_id,
        note: e.note,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}

fn pii_access_log_to_dto(e: PiiAccessLog) -> Result<PiiAccessLogDto> {
    Ok(PiiAccessLogDto {
        id: PiiAccessLogId(e.id),
        company_id: e.company_id,
        employee_id: e.employee_id,
        accessed_by: e.accessed_by,
        data_category: e.data_category,
        purpose: e.purpose,
        accessed_at: e.accessed_at,
    })
}

fn religion_to_dto(e: Religion) -> Result<ReligionDto> {
    Ok(ReligionDto {
        id: ReligionId(e.id),
        name: e.name,
        metadata: serde_json::to_value(&e.metadata)?,
    })
}
