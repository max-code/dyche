-- Add migration script here
ALTER TABLE players ADD CONSTRAINT fk_club FOREIGN KEY (team) REFERENCES clubs (id);
