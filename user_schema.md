# User Schema

## Table

```sql
DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD username ON TABLE user TYPE string;
DEFINE FIELD password ON TABLE user TYPE string;
DEFINE FIELD email ON TABLE user TYPE string;
DEFINE FIELD created_at ON TABLE user TYPE datetime;
```

## Insert Single

```sql
INSERT INTO user {
    username: "jokar",
    password: "Thomas1234!",
    email: "jon@gmail.com",
    created_at: time::now()
};

-- OR

CREATE user CONTENT {
	username: "jokar",
    password: "Thomas1234!",
    email: "jon@gmail.com",
    created_at: time::now()
};
```

## Insert Multiple

```sql
INSERT INTO user [
    {
        username:  "peter",
        password: "CarrotEater1234!",
        email: "peter@gmail.com",
        created_at: time::now()
    },
    {
        username:  "jon",
        password: "Thomas1234!",
        email: "jon@gmail.com",
        created_at: time::now()
    },
];
```
