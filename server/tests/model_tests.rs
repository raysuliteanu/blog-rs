use diesel::PgConnection;
use server::{db, model};

#[test]
fn create_users() {
    let mut conn: PgConnection = db::establish_connection();

    let alice = model::User::create_user("Alice", &mut conn).expect("create user Alice");
    let bob = model::User::create_user("Bob", &mut conn).expect("create user Bob");
    let carol = model::User::create_user("Carol", &mut conn).expect("create user Carol");

    assert_eq!(
        "Alice",
        model::User::find_user_by_name(&alice.username, &mut conn)
            .unwrap()
            .username
    );

    assert_eq!(
        "Bob",
        model::User::find_user_by_name(&bob.username, &mut conn)
            .unwrap()
            .username
    );

    assert_eq!(
        "Carol",
        model::User::find_user_by_name(&carol.username, &mut conn)
            .unwrap()
            .username
    );
}

#[test]
#[ignore = "disable this until set up a way to reset the database each test run"]
fn get_all_users() {
    let mut conn: PgConnection = db::establish_connection();
    let users = model::User::get_users(&mut conn).expect("get all users");
    assert_eq!(users.len(), 3);
}

#[test]
fn create_blog() {
    let mut conn: PgConnection = db::establish_connection();

    let alice = model::User::find_user_by_name("Alice", &mut conn).expect("find user Alice");
    let _alice_blog = model::create_blog(&alice, "Alice's blog", "blog by Alice", &mut conn)
        .expect("create Alice's blog");

    let bob = model::User::find_user_by_name("Bob", &mut conn).expect("find user Bob");
    let _bob_blog = model::create_blog(&bob, "Bob's blog", "blog by Bob", &mut conn)
        .expect("create Bob's blog");

    let carol = model::User::find_user_by_name("Carol", &mut conn).expect("find user Carol");
    let _carol_blog = model::create_blog(&carol, "Carol's blog", "blog by Carol", &mut conn)
        .expect("create Carol's blog");
}

#[test]
#[ignore = "disable this until set up a way to reset the database each test run"]
fn get_all_blogs() {
    let mut conn: PgConnection = db::establish_connection();
    let blogs = model::get_all_blogs(&mut conn).unwrap();
    assert_eq!(blogs.len(), 3);
}

#[test]
fn create_post() {
    let mut conn: PgConnection = db::establish_connection();
    let alice_blog = model::get_blog(2, &mut conn).expect("find Alic's blog");
    let _p1 = model::create_post(
        &alice_blog,
        "the first post",
        "whatever",
        "some content",
        &mut conn,
    )
    .expect("create post 1");

    let _p2 = model::create_post(
        &alice_blog,
        "the second post",
        "yada yada",
        "yada yada content",
        &mut conn,
    )
    .expect("create post 2");
}

#[test]
#[ignore = "disable this until set up a way to reset the database each test run"]
fn get_posts() {
    let mut conn: PgConnection = db::establish_connection();
    let alice_blog = model::get_blog(2, &mut conn).expect("find Alic's blog");
    let results = model::get_posts(&alice_blog, &mut conn).unwrap();
    assert_eq!(results.len(), 2);
}
