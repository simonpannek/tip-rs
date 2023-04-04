pub use sea_orm_migration::prelude::*;

mod m20230226_213739_create_initial_tables;
mod m20230321_070938_add_settings;
mod m20230327_161531_add_basic_event_fields;
mod m20230327_192518_add_user_data_fields;
mod m20230402_132850_add_scheduled_actions;
mod m20230403_204248_add_survey_response_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230226_213739_create_initial_tables::Migration),
            Box::new(m20230321_070938_add_settings::Migration),
            Box::new(m20230327_161531_add_basic_event_fields::Migration),
            Box::new(m20230327_192518_add_user_data_fields::Migration),
            Box::new(m20230402_132850_add_scheduled_actions::Migration),
            Box::new(m20230403_204248_add_survey_response_table::Migration),
        ]
    }
}
