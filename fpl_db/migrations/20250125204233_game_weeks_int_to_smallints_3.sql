-- Add migration script here
ALTER TABLE game_weeks
ALTER COLUMN most_selected TYPE smallint,
ALTER COLUMN most_transferred_in TYPE smallint,
ALTER COLUMN top_element TYPE smallint,
ALTER COLUMN most_captained TYPE smallint,
ALTER COLUMN most_vice_captained TYPE smallint;
