pub mod models;

use jiff::civil::DateTime;
use models::{
    AutoproxySettings, Member, MemberRef, PublicSystemSettings, System, SystemGuildSettings,
    SystemRef, SystemSettings,
};
use reqwest::{Client, Method, Request, RequestBuilder, Response, StatusCode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::models::{
    Group, GroupRef, MemberGuildSettings, Message, PluralKitUuid, Switch, SwitchWithMembers,
    marker::SwitchMarker,
};

type Error = Box<dyn std::error::Error>;

pub struct PluralKit {
    client: Client,
}

impl PluralKit {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent(format!("pluralkit-rs/{}", env!("CARGO_PKG_VERSION")))
                .build()
                .expect("error building reqwest client"),
        }
    }

    // system
    pub async fn get_system(&self, system_ref: SystemRef) -> Result<System, Error> {
        todo!()
    }

    pub async fn update_system(&self, system: System) -> Result<System, Error> {
        todo!()
    }

    pub async fn get_my_system_settings(&self) -> Result<SystemSettings, Error> {
        todo!()
    }

    pub async fn get_system_settings(
        &self,
        system_ref: SystemRef,
    ) -> Result<PublicSystemSettings, Error> {
        todo!()
    }

    pub fn update_system_settings(
        &self,
        settings: SystemSettings,
    ) -> Result<SystemSettings, Error> {
        todo!()
    }

    pub fn get_system_guild_settings(
        &self,
        guild_id: String,
    ) -> Result<SystemGuildSettings, Error> {
        todo!()
    }

    pub fn update_system_guild_settings(
        &self,
        settings: SystemGuildSettings,
    ) -> Result<SystemGuildSettings, Error> {
        todo!()
    }

    pub async fn get_system_autoproxy_settings(&self) -> Result<AutoproxySettings, Error> {
        todo!()
    }

    pub fn update_system_autoproxy_settings(
        &self,
        guild_id: String,
        settings: AutoproxySettings,
    ) -> Result<AutoproxySettings, Error> {
        todo!()
    }

    // member
    pub async fn get_system_members(&self, system_ref: SystemRef) -> Result<Vec<Member>, Error> {
        todo!()
    }

    pub async fn create_member(&self, member: Member) -> Result<Member, Error> {
        todo!()
    }

    pub async fn get_member(&self, member_ref: MemberRef) -> Result<Member, Error> {
        todo!()
    }

    pub async fn update_member(&self, member: Member) -> Result<Member, Error> {
        todo!()
    }

    pub async fn delete_member(&self, member_ref: MemberRef) -> Result<(), Error> {
        todo!()
    }

    pub async fn get_member_groups(&self, member_ref: MemberRef) -> Result<Vec<Group>, Error> {
        todo!()
    }

    pub fn add_member_to_groups(
        &self,
        member_ref: MemberRef,
        groups: Vec<GroupRef>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn remove_member_from_groups(
        &self,
        member_ref: MemberRef,
        groups: Vec<GroupRef>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn overwrite_member_groups(
        &self,
        member_ref: MemberRef,
        groups: Vec<GroupRef>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn get_member_guild_settings(
        &self,
        member_ref: MemberRef,
        guild_id: String,
    ) -> Result<MemberGuildSettings, Error> {
        todo!()
    }

    pub fn update_member_guild_settings(
        &self,
        member_ref: MemberRef,
        guild_id: String,
        settings: MemberGuildSettings,
    ) -> Result<MemberGuildSettings, Error> {
        todo!()
    }

    // group
    pub async fn get_system_groups(&self, system_ref: SystemRef) -> Result<Vec<Group>, Error> {
        todo!()
    }

    pub async fn create_group(&self, group: Group) -> Result<Group, Error> {
        todo!()
    }

    pub async fn get_group(&self, group_ref: GroupRef) -> Result<Group, Error> {
        todo!()
    }

    pub async fn update_group(&self, group: Group) -> Result<Group, Error> {
        todo!()
    }

    pub async fn delete_group(&self, group_ref: GroupRef) -> Result<(), Error> {
        todo!()
    }

    pub async fn get_group_members(&self, group_ref: GroupRef) -> Result<Vec<Member>, Error> {
        todo!()
    }

    pub fn add_members_to_group(
        &self,
        group_ref: GroupRef,
        member_refs: Vec<MemberRef>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn remove_members_from_group(
        &self,
        group_ref: GroupRef,
        member_refs: Vec<MemberRef>,
    ) -> Result<(), Error> {
        todo!()
    }

    pub fn overwrite_group_members(
        &self,
        group_ref: GroupRef,
        member_refs: Vec<MemberRef>,
    ) -> Result<(), Error> {
        todo!()
    }

    // switches
    pub async fn get_system_switches(&self, system_ref: SystemRef) -> Result<Vec<Switch>, Error> {
        todo!()
    }

    pub fn get_current_system_fronters(
        &self,
        system_ref: SystemRef,
    ) -> Result<SwitchWithMembers, Error> {
        todo!()
    }

    pub fn create_switch(
        &self,
        system_ref: &SystemRef,
        members: Vec<MemberRef>,
        timestamp: Option<DateTime>,
    ) -> Result<SwitchWithMembers, Error> {
        todo!()
    }

    pub fn get_switch(
        &self,
        system_ref: &SystemRef,
        switch_ref: PluralKitUuid<SwitchMarker>,
    ) -> Result<SwitchWithMembers, Error> {
        todo!()
    }

    pub fn update_switch(
        &self,
        system_ref: &SystemRef,
        switch_ref: PluralKitUuid<SwitchMarker>,
        timestamp: DateTime,
    ) -> Result<SwitchWithMembers, Error> {
        todo!()
    }

    pub fn update_switch_members(
        &self,
        system_ref: &SystemRef,
        switch_ref: PluralKitUuid<SwitchMarker>,
        members: Vec<MemberRef>,
    ) -> Result<SwitchWithMembers, Error> {
        todo!()
    }

    pub async fn delete_switch(
        &self,
        system_ref: &SystemRef,
        switch_ref: PluralKitUuid<SwitchMarker>,
    ) -> Result<(), Error> {
        todo!()
    }

    // misc
    pub async fn get_proxied_message_information(
        &self,
        message_id: String,
    ) -> Result<Message, Error> {
        todo!()
    }

    // handlers
    async fn send(&self, builder: RequestBuilder) -> Result<Response, Error> {
        let resp = builder.send().await?.error_for_status()?;
        Ok(resp)
    }

    async fn send_expect_204(&self, builder: RequestBuilder) -> Result<(), Error> {
        let response = self.send(builder).await?;

        (response.status() != StatusCode::NO_CONTENT).ok_or(
            format!(
                "expected status code 204 but received {}",
                response.status().as_u16(),
            )
            .into(),
        )
    }

    async fn send_json<RT: DeserializeOwned>(&self, builder: RequestBuilder) -> Result<RT, Error> {
        Ok(self.send(builder).await?.json().await?)
    }
}
