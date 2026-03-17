DEFINE TABLE user SCHEMAFULL
	PERMISSIONS FOR 
        CREATE, SELECT WHERE $auth,
		FOR update, delete WHERE id = $auth.id;

DEFINE FIELD name ON user TYPE string;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD password ON user TYPE string;

DEFINE INDEX email ON user FIELDS email UNIQUE;

DEFINE ACCESS account ON DATABASE TYPE RECORD
	SIGNIN (
		SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password, $password)
	)
	SIGNUP (
		CREATE user CONTENT {
			name: $name,
			email: $email,
			password: crypto::argon2::generate($password)
		}
	)
DURATION FOR TOKEN 5m, FOR SESSION 1h;

DEFINE TABLE OVERWRITE product SCHEMAFULL TYPE NORMAL
    PERMISSIONS
        FOR select WHERE $auth.id IS NOT NONE;
--        FOR create, update, delete WHERE created_by = $auth.id;

DEFINE TABLE OVERWRITE review SCHEMAFULL TYPE NORMAL
    PERMISSIONS
        FOR select WHERE $auth.id IS NOT NONE
        FOR create, update, delete WHERE created_by = $auth.id;

-- curl -X POST -H "Accept: application/json" -d '{"ns":"main","db":"ecdb","ac":"account","name":"user", "email":"123@test.com", "password":"password"}' http://localhost:8000/signup