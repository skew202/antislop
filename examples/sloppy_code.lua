-- This is a simple example of AI-generated slop code

function processData(data)
    -- TODO: implement proper validation
    -- for now we just return the data
    local result = data

    -- This is a quick implementation for testing
    -- hopefully this works correctly
    if data then
        -- temporary hack to fix the issue
        result = data:gsub("^%s+", ""):gsub("%s+$", "")
    end

    -- FIXME: add error handling later
    -- in a real world scenario we would use a proper parser
    return result
end

-- TODO: add more functions
-- TODO: write tests
-- NOTE: this is important - remember to refactor

-- stub: not implemented
function getUser(id)
    -- TODO: implement this
    -- basically just return nil for now
    return nil
end
