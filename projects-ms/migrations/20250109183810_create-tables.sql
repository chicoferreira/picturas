CREATE TABLE IF NOT EXISTS projects
(
    id         UUID PRIMARY KEY,
    name       VARCHAR(255)                          NOT NULL,
    user_id    UUID                                  NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS images
(
    id         UUID PRIMARY KEY,
    name       VARCHAR(255) NOT NULL,
    project_id UUID         NOT NULL REFERENCES projects (id)
);

CREATE TABLE IF NOT EXISTS tools
(
    id         UUID PRIMARY KEY,
    project_id UUID         NOT NULL REFERENCES projects (id),
    position   INTEGER      NOT NULL,
    procedure  VARCHAR(255) NOT NULL,
    parameters JSONB        NOT NULL
);

CREATE TABLE IF NOT EXISTS image_versions
(
    id                UUID PRIMARY KEY,
    project_id        UUID                                  NOT NULL REFERENCES projects (id),
    original_image_id UUID                                  NOT NULL REFERENCES images (id),
    tool_id           UUID                                  NOT NULL REFERENCES tools (id),
    text_result       TEXT, -- OCR results, if the tool is OCR, or any textual output
    created_at        TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);