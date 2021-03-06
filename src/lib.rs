mod jni_c_header;
use ironoxide::{
    document::{
        AssociationType, DocAccessEditErr, DocumentAccessResult, DocumentDecryptResult,
        DocumentEncryptOpts, DocumentEncryptResult, DocumentListMeta, DocumentListResult,
        DocumentMetadataResult, UserOrGroup, VisibleGroup, VisibleUser,
    },
    group::{
        GroupAccessEditErr, GroupAccessEditResult, GroupCreateOpts, GroupGetResult,
        GroupListResult, GroupMetaResult,
    },
    prelude::*,
    user::{
        DeviceCreateOpts, UserCreateKeyPair, UserDevice, UserDeviceListResult, UserVerifyResult,
    },
};
use ironoxide::{DeviceContext, DeviceSigningKeyPair, PrivateKey, PublicKey};

include!(concat!(env!("OUT_DIR"), "/lib.rs"));

#[derive(Clone)]
pub struct UserWithKey((UserId, PublicKey));
impl UserWithKey {
    pub fn user(&self) -> UserId {
        (self.0).0.clone()
    }

    pub fn public_key(&self) -> PublicKey {
        (self.0).1.clone()
    }
}

fn i8_conv(i8s: &[i8]) -> &[u8] {
    unsafe { core::slice::from_raw_parts(i8s.as_ptr() as *const u8, i8s.len()) }
}

fn u8_conv(u8s: &[u8]) -> &[i8] {
    unsafe { core::slice::from_raw_parts(u8s.as_ptr() as *const i8, u8s.len()) }
}

mod visible_user {
    use super::*;
    pub fn id(d: &VisibleUser) -> UserId {
        d.id().clone()
    }
}

mod visible_group {
    use super::*;
    pub fn id(d: &VisibleGroup) -> GroupId {
        d.id().clone()
    }
    pub fn name(d: &VisibleGroup) -> Option<GroupName> {
        d.name().cloned()
    }
}

mod user_id {
    use super::*;
    use std::convert::TryInto;
    pub fn id(u: &UserId) -> String {
        u.id().clone()
    }

    pub fn validate(s: &str) -> Result<UserId, String> {
        Ok(s.try_into()?)
    }
}

mod group_id {
    use super::*;
    use std::convert::TryInto;
    pub fn id(g: &GroupId) -> String {
        g.id().clone()
    }

    pub fn validate(s: &str) -> Result<GroupId, String> {
        Ok(s.try_into()?)
    }
}

mod group_name {
    use super::*;
    use std::convert::TryInto;
    pub fn name(g: &GroupName) -> String {
        g.name().clone()
    }
    pub fn validate(g: &str) -> Result<GroupName, String> {
        Ok(g.try_into()?)
    }
}

mod document_id {
    use super::*;
    use std::convert::TryInto;
    pub fn id(d: &DocumentId) -> String {
        d.id().clone()
    }
    pub fn validate(s: &str) -> Result<DocumentId, String> {
        Ok(s.try_into()?)
    }
}

mod document_name {
    use super::*;
    use std::convert::TryInto;
    pub fn name(d: &DocumentName) -> String {
        d.name().clone()
    }
    pub fn validate(d: &str) -> Result<DocumentName, String> {
        Ok(d.try_into()?)
    }
}

mod device_id {
    use super::*;
    use std::convert::TryInto;
    pub fn id(d: &DeviceId) -> i64 {
        //By constructon, DeviceIds are validated to be at most i64 max so this value won't
        //wrap over to be negative
        d.id().clone() as i64
    }
    pub fn validate(s: i64) -> Result<DeviceId, String> {
        Ok((s as u64).try_into()?)
    }
}

mod device_name {
    use super::*;
    use std::convert::TryInto;
    pub fn name(d: &DeviceName) -> String {
        d.name().clone()
    }
    pub fn validate(n: &str) -> Result<DeviceName, String> {
        Ok(n.try_into()?)
    }
}

mod public_key {
    use super::*;
    pub fn as_bytes(pk: &PublicKey) -> Vec<i8> {
        u8_conv(&pk.as_bytes()[..]).to_vec()
    }
}

mod private_key {
    use super::*;
    use std::convert::TryInto;
    pub fn validate(bytes: &[i8]) -> Result<PrivateKey, String> {
        Ok(i8_conv(bytes).try_into()?)
    }
    pub fn as_bytes(pk: &PrivateKey) -> Vec<i8> {
        u8_conv(&pk.as_bytes()[..]).to_vec()
    }
}

mod device_signing_keys {
    use super::*;
    use std::convert::TryInto;
    pub fn validate(bytes: &[i8]) -> Result<DeviceSigningKeyPair, String> {
        Ok(i8_conv(bytes).try_into()?)
    }
    pub fn as_bytes(pk: &DeviceSigningKeyPair) -> Vec<i8> {
        u8_conv(&pk.as_bytes()[..]).to_vec()
    }
}

mod device_create_opt {
    use super::*;
    pub fn create(name: Option<DeviceName>) -> DeviceCreateOpts {
        DeviceCreateOpts::new(name)
    }
}

mod document_create_opt {
    use super::*;
    use ironoxide::document::DocumentEncryptOpts;
    pub fn create(
        id: Option<DocumentId>,
        name: Option<DocumentName>,
        user_grants: Vec<UserId>,
        group_grants: Vec<GroupId>,
    ) -> DocumentEncryptOpts {
        let users_and_groups = user_grants
            .into_iter()
            .map(|u| UserOrGroup::User { id: u })
            .chain(
                group_grants
                    .into_iter()
                    .map(|g| UserOrGroup::Group { id: g }),
            )
            .collect();

        DocumentEncryptOpts::new(id, name, users_and_groups)
    }
}

mod device_context {
    use super::*;
    pub fn new(
        account_id: &UserId,
        segment_id: i64,
        private_key: &PrivateKey,
        signing_keys: &DeviceSigningKeyPair,
    ) -> DeviceContext {
        DeviceContext::new(
            account_id.clone(),
            segment_id as usize,
            private_key.clone(),
            signing_keys.clone(),
        )
    }
    pub fn account_id(d: &DeviceContext) -> UserId {
        d.account_id().clone()
    }

    pub fn segment_id(d: &DeviceContext) -> usize {
        d.segment_id()
    }

    pub fn private_device_key(d: &DeviceContext) -> PrivateKey {
        d.private_device_key().clone()
    }

    pub fn signing_keys(d: &DeviceContext) -> DeviceSigningKeyPair {
        d.signing_keys().clone()
    }
}

mod user_create_key_pair {
    use super::*;
    pub fn user_encrypted_master_key_bytes(u: &UserCreateKeyPair) -> Vec<i8> {
        u8_conv(&u.user_encrypted_master_key_bytes()[..]).to_vec()
    }

    pub fn user_public_key(u: &UserCreateKeyPair) -> PublicKey {
        u.user_public_key().clone()
    }
}

mod user_verify_result {
    use super::*;
    pub fn user_public_key(u: &UserVerifyResult) -> PublicKey {
        u.user_public_key().clone()
    }

    pub fn account_id(u: &UserVerifyResult) -> UserId {
        u.account_id().clone()
    }

    pub fn segment_id(u: &UserVerifyResult) -> usize {
        u.segment_id()
    }
}

mod user_device {
    use super::*;
    pub fn id(d: &UserDevice) -> DeviceId {
        d.id().clone()
    }

    pub fn name(d: &UserDevice) -> Option<DeviceName> {
        d.name().cloned()
    }

    pub fn created(d: &UserDevice) -> DateTime<Utc> {
        d.created().clone()
    }

    pub fn last_updated(d: &UserDevice) -> DateTime<Utc> {
        d.last_updated().clone()
    }
}

mod user_device_list_result {
    use super::*;
    pub fn result(d: &UserDeviceListResult) -> Vec<UserDevice> {
        d.result().clone()
    }
}

mod document_list_result {
    use super::*;
    pub fn result(d: &DocumentListResult) -> Vec<DocumentListMeta> {
        d.result().iter().map(|a| a.clone()).collect()
    }
}

mod document_list_meta {
    use super::*;
    pub fn id(d: &DocumentListMeta) -> DocumentId {
        d.id().clone()
    }
    pub fn name(d: &DocumentListMeta) -> Option<DocumentName> {
        d.name().cloned()
    }
    pub fn association_type(d: &DocumentListMeta) -> AssociationType {
        d.association_type().clone()
    }
    pub fn created(d: &DocumentListMeta) -> DateTime<Utc> {
        d.created().clone()
    }
    pub fn last_updated(d: &DocumentListMeta) -> DateTime<Utc> {
        d.last_updated().clone()
    }
}

mod document_metadata_result {
    use super::*;
    pub fn id(d: &DocumentMetadataResult) -> DocumentId {
        d.id().clone()
    }
    pub fn name(d: &DocumentMetadataResult) -> Option<DocumentName> {
        d.name().cloned()
    }
    pub fn created(d: &DocumentMetadataResult) -> DateTime<Utc> {
        d.created().clone()
    }
    pub fn last_updated(d: &DocumentMetadataResult) -> DateTime<Utc> {
        d.last_updated().clone()
    }
    pub fn association_type(d: &DocumentMetadataResult) -> AssociationType {
        d.association_type().clone()
    }
    pub fn visible_to_users(d: &DocumentMetadataResult) -> Vec<VisibleUser> {
        d.visible_to_users().clone()
    }
    pub fn visible_to_groups(d: &DocumentMetadataResult) -> Vec<VisibleGroup> {
        d.visible_to_groups().clone()
    }
}

mod document_encrypt_result {
    use super::*;

    pub fn id(d: &DocumentEncryptResult) -> DocumentId {
        d.id().clone()
    }
    pub fn name(d: &DocumentEncryptResult) -> Option<DocumentName> {
        d.name().cloned()
    }
    pub fn created(d: &DocumentEncryptResult) -> DateTime<Utc> {
        d.created().clone()
    }
    pub fn last_updated(d: &DocumentEncryptResult) -> DateTime<Utc> {
        d.last_updated().clone()
    }
    pub fn encrypted_data(d: &DocumentEncryptResult) -> Vec<i8> {
        u8_conv(d.encrypted_data()).to_vec()
    }
}

mod document_decrypt_result {
    use super::*;

    pub fn id(d: &DocumentDecryptResult) -> DocumentId {
        d.id().clone()
    }
    pub fn name(d: &DocumentDecryptResult) -> Option<DocumentName> {
        d.name().cloned()
    }
    pub fn created(d: &DocumentDecryptResult) -> DateTime<Utc> {
        d.created().clone()
    }
    pub fn last_updated(d: &DocumentDecryptResult) -> DateTime<Utc> {
        d.last_updated().clone()
    }
    pub fn decrypted_data(d: &DocumentDecryptResult) -> Vec<i8> {
        u8_conv(d.decrypted_data()).to_vec()
    }
}

// UserAccessErr and GroupAccessErr are a Java-compatible representation of IronOxide's
// DocAccessEditErr. They are encoded this this because this seemed like the most
// straightforward way to represent a error for both a user or group (like UserOrGroup)
#[derive(Debug, Clone, PartialEq)]
pub struct UserAccessErr {
    id: UserId,
    err: String,
}

impl UserAccessErr {
    pub fn id(&self) -> UserId {
        self.id.clone()
    }

    pub fn err(&self) -> String {
        self.err.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupAccessErr {
    id: GroupId,
    err: String,
}

impl GroupAccessErr {
    pub fn id(&self) -> GroupId {
        self.id.clone()
    }

    pub fn err(&self) -> String {
        self.err.clone()
    }
}

mod document_access_change_result {
    use super::*;
    use itertools::{Either, Itertools};

    pub struct SucceededResult {
        users: Vec<UserId>,
        groups: Vec<GroupId>,
    }

    impl SucceededResult {
        pub fn users(&self) -> Vec<UserId> {
            self.users.clone()
        }
        pub fn groups(&self) -> Vec<GroupId> {
            self.groups.clone()
        }
    }

    pub struct FailedResult {
        users: Vec<UserAccessErr>,
        groups: Vec<GroupAccessErr>,
    }

    impl FailedResult {
        pub fn is_empty(&self) -> bool {
            self.users.is_empty() && self.groups.is_empty()
        }

        pub fn users(&self) -> Vec<UserAccessErr> {
            self.users.clone()
        }

        pub fn groups(&self) -> Vec<GroupAccessErr> {
            self.groups.clone()
        }
    }

    pub trait DocumentAccessChange {
        fn changed(&self) -> SucceededResult;
        fn errors(&self) -> FailedResult;
    }

    impl DocumentAccessChange for DocumentAccessResult {
        fn changed(&self) -> SucceededResult {
            to_succeeded_result(self.succeeded())
        }

        fn errors(&self) -> FailedResult {
            to_failed_result(self.failed())
        }
    }

    impl DocumentAccessChange for DocumentEncryptResult {
        fn changed(&self) -> SucceededResult {
            to_succeeded_result(self.grants())
        }

        fn errors(&self) -> FailedResult {
            to_failed_result(self.access_errs())
        }
    }

    fn to_succeeded_result(successes: &[UserOrGroup]) -> SucceededResult {
        let (users, groups) = successes.iter().cloned().partition_map(|uog| match uog {
            UserOrGroup::User { id } => Either::Left(id),
            UserOrGroup::Group { id } => Either::Right(id),
        });

        SucceededResult { users, groups }
    }

    fn to_failed_result(access_errs: &[DocAccessEditErr]) -> FailedResult {
        let (users, groups) =
            access_errs
                .iter()
                .cloned()
                .partition_map(|access_err| match access_err {
                    DocAccessEditErr {
                        user_or_group: UserOrGroup::User { id },
                        err,
                    } => Either::Left(UserAccessErr { id: id, err: err }),
                    DocAccessEditErr {
                        user_or_group: UserOrGroup::Group { id },
                        err,
                    } => Either::Right(GroupAccessErr { id: id, err: err }),
                });

        FailedResult { users, groups }
    }
}

mod group_meta_result {
    use super::*;
    pub fn id(g: &GroupMetaResult) -> GroupId {
        g.id().clone()
    }
    pub fn name(g: &GroupMetaResult) -> Option<GroupName> {
        g.name().cloned()
    }
    pub fn created(g: &GroupMetaResult) -> DateTime<Utc> {
        g.created().clone()
    }
    pub fn last_updated(g: &GroupMetaResult) -> DateTime<Utc> {
        g.last_updated().clone()
    }
}

mod group_list_result {
    use super::*;
    pub fn result(g: &GroupListResult) -> Vec<GroupMetaResult> {
        g.result().clone()
    }
}

mod group_get_result {
    use super::*;
    /// Wrap the Vec<UserId> type in a newtype because swig can't handle
    /// passing through an Option<Vec<*>>
    pub struct GroupUserList(Vec<UserId>);
    impl GroupUserList {
        pub fn list(&self) -> Vec<UserId> {
            self.0.clone()
        }
    }

    pub fn id(g: &GroupGetResult) -> GroupId {
        g.id().clone()
    }
    pub fn name(g: &GroupGetResult) -> Option<GroupName> {
        g.name().cloned()
    }
    pub fn group_master_public_key(result: &GroupGetResult) -> PublicKey {
        result.group_master_public_key().clone()
    }
    pub fn admin_list(result: &GroupGetResult) -> Option<GroupUserList> {
        result.admin_list().cloned().map(GroupUserList)
    }
    pub fn member_list(result: &GroupGetResult) -> Option<GroupUserList> {
        result.member_list().cloned().map(GroupUserList)
    }
    pub fn created(g: &GroupGetResult) -> DateTime<Utc> {
        g.created().clone()
    }
    pub fn last_updated(g: &GroupGetResult) -> DateTime<Utc> {
        g.last_updated().clone()
    }
}

mod group_create_opts {
    use super::*;
    pub fn create(
        id: Option<GroupId>,
        name: Option<GroupName>,
        add_as_member: bool,
    ) -> GroupCreateOpts {
        GroupCreateOpts::new(id, name, add_as_member)
    }
}

//Java SDK wrapper functions for doing unnatural things with the JNI.
fn user_verify(jwt: &str) -> Result<Option<UserVerifyResult>, String> {
    Ok(IronOxide::user_verify(jwt)?)
}
fn user_create(jwt: &str, password: &str) -> Result<UserCreateKeyPair, String> {
    Ok(IronOxide::user_create(jwt, password)?)
}
fn initialize(init: &DeviceContext) -> Result<IronOxide, String> {
    Ok(ironoxide::initialize(init)?)
}
fn generate_new_device(
    jwt: &str,
    password: &str,
    opts: &DeviceCreateOpts,
) -> Result<DeviceContext, String> {
    Ok(IronOxide::generate_new_device(jwt, password, opts)?)
}
fn user_list_devices(sdk: &IronOxide) -> Result<UserDeviceListResult, String> {
    Ok(sdk.user_list_devices()?)
}
fn user_get_public_key(sdk: &IronOxide, users: Vec<UserId>) -> Result<Vec<UserWithKey>, String> {
    let users = &users.into_iter().map(|s| s.into()).collect::<Vec<_>>();
    let mut result = sdk.user_get_public_key(users)?;
    Ok(result.drain().into_iter().map(UserWithKey).collect())
}
fn user_delete_device(sdk: &IronOxide, device_id: Option<DeviceId>) -> Result<DeviceId, String> {
    Ok(sdk.user_delete_device(device_id.as_ref())?)
}
fn document_list(sdk: &IronOxide) -> Result<DocumentListResult, String> {
    Ok(sdk.document_list()?)
}
fn document_get_metadata(
    sdk: &IronOxide,
    id: &DocumentId,
) -> Result<DocumentMetadataResult, String> {
    Ok(sdk.document_get_metadata(id)?)
}
fn document_get_id_from_bytes(sdk: &IronOxide, bytes: &[i8]) -> Result<DocumentId, String> {
    Ok(sdk.document_get_id_from_bytes(i8_conv(bytes))?)
}
fn document_encrypt(
    sdk: &mut IronOxide,
    data: &[i8],
    opts: &DocumentEncryptOpts,
) -> Result<DocumentEncryptResult, String> {
    Ok(sdk.document_encrypt(i8_conv(data), opts)?)
}
fn document_update_bytes(
    sdk: &mut IronOxide,
    document_id: &DocumentId,
    data: &[i8],
) -> Result<DocumentEncryptResult, String> {
    Ok(sdk.document_update_bytes(document_id, i8_conv(data))?)
}
fn document_decrypt(sdk: &mut IronOxide, data: &[i8]) -> Result<DocumentDecryptResult, String> {
    Ok(sdk.document_decrypt(i8_conv(data))?)
}
fn document_update_name(
    sdk: &IronOxide,
    document_id: &DocumentId,
    name: Option<DocumentName>,
) -> Result<DocumentMetadataResult, String> {
    Ok(sdk.document_update_name(document_id, name.as_ref())?)
}

fn document_grant_access(
    sdk: &mut IronOxide,
    document_id: &DocumentId,
    grant_users: Vec<UserId>,
    grant_groups: Vec<GroupId>,
) -> Result<DocumentAccessResult, String> {
    let users_and_groups = grant_users
        .into_iter()
        .map(|u| UserOrGroup::User { id: u })
        .chain(
            grant_groups
                .into_iter()
                .map(|g| UserOrGroup::Group { id: g }),
        )
        .collect();

    Ok(sdk.document_grant_access(document_id, &users_and_groups)?)
}

fn document_revoke_access(
    sdk: &IronOxide,
    document_id: &DocumentId,
    revoke_users: Vec<UserId>,
    revoke_groups: Vec<GroupId>,
) -> Result<DocumentAccessResult, String> {
    let users_and_groups = revoke_users
        .into_iter()
        .map(|u| UserOrGroup::User { id: u })
        .chain(
            revoke_groups
                .into_iter()
                .map(|g| UserOrGroup::Group { id: g }),
        )
        .collect();

    Ok(sdk.document_revoke_access(document_id, &users_and_groups)?)
}
fn group_list(sdk: &IronOxide) -> Result<GroupListResult, String> {
    Ok(sdk.group_list()?)
}
fn group_get_metadata(sdk: &IronOxide, id: &GroupId) -> Result<GroupGetResult, String> {
    Ok(sdk.group_get_metadata(id)?)
}
fn group_create(sdk: &mut IronOxide, opts: &GroupCreateOpts) -> Result<GroupMetaResult, String> {
    Ok(sdk.group_create(opts)?)
}
fn group_update_name(
    sdk: &IronOxide,
    id: &GroupId,
    name: Option<GroupName>,
) -> Result<GroupMetaResult, String> {
    Ok(sdk.group_update_name(id, name.as_ref())?)
}
fn group_delete(sdk: &IronOxide, id: &GroupId) -> Result<GroupId, String> {
    Ok(sdk.group_delete(id)?)
}
fn group_add_members(
    sdk: &mut IronOxide,
    group_id: &GroupId,
    users: Vec<UserId>,
) -> Result<GroupAccessEditResult, String> {
    Ok(sdk.group_add_members(group_id, &users)?)
}
fn group_remove_members(
    sdk: &IronOxide,
    group_id: &GroupId,
    users: Vec<UserId>,
) -> Result<GroupAccessEditResult, String> {
    Ok(sdk.group_remove_members(group_id, &users)?)
}
fn group_add_admins(
    sdk: &mut IronOxide,
    group_id: &GroupId,
    users: Vec<UserId>,
) -> Result<GroupAccessEditResult, String> {
    Ok(sdk.group_add_admins(group_id, &users)?)
}
fn group_remove_admins(
    sdk: &IronOxide,
    group_id: &GroupId,
    users: Vec<UserId>,
) -> Result<GroupAccessEditResult, String> {
    Ok(sdk.group_remove_admins(group_id, &users)?)
}

mod group_access_edit_result {
    use super::*;
    pub fn succeeded(result: &GroupAccessEditResult) -> Vec<UserId> {
        result.succeeded().clone()
    }

    pub fn failed(result: &GroupAccessEditResult) -> Vec<GroupAccessEditErr> {
        result.failed().to_vec()
    }
}

mod access_edit_failure {
    use super::*;
    pub fn user(result: &GroupAccessEditErr) -> UserId {
        result.user().clone()
    }

    pub fn error(result: &GroupAccessEditErr) -> String {
        result.error().clone()
    }
}
