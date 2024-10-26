use diesel::{
    data_types::PgTimestamp,
    helper_types::Limit,
    prelude::*,
    query_dsl::{
        methods::LimitDsl,
        LoadQuery,
    },
};
use crate::db::{
    establish_connection,
    schema,
};

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}

#[derive(Insertable)]
#[diesel(table_name = schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertPost {
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn first<Q>(query: impl Fn(schema::posts::table) -> Q) -> QueryResult<Self>
    where
        Q: LimitDsl + RunQueryDsl<PgConnection>,
        Limit<Q>: LoadQuery<'static, PgConnection, Self>,
    {
        use schema::posts::dsl::*;

        let conn = &mut establish_connection();
        query(posts).first(conn)
    }

    pub fn load<Q>(query: impl Fn(schema::posts::table) -> Q) -> QueryResult<Vec<Self>>
    where
        Q: LoadQuery<'static, PgConnection, Self> + RunQueryDsl<PgConnection>,
    {
        use schema::posts::dsl::*;

        let conn = &mut establish_connection();
        query(posts).load(conn)
    }
}
