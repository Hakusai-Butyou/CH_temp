DEFINE NAMESPACE main;
DEFINE DATABASE main;
USE NS main DB main;


DEFINE TABLE Account SCHEMAFULL;

DEFINE FIELD name ON Account TYPE string ASSERT $value != NONE;
DEFINE FIELD google_id ON Account TYPE string ASSERT $value != NONE;
DEFINE FIELD created_at ON Account TYPE datetime ASSERT $value != NONE DEFAULT time::now();

DEFINE INDEX G_id ON Account FIELDS google_id UNIQUE;