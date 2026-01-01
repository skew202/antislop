// Example JavaScript code with detected AI shortcuts

function processUser(user) {
    if (!user) return false;

    try {
        saveUser(user);
    } catch (e) {
        // empty catch - "make it run"
    }

    // TODO: add actual validation
    return true;
}

function saveUser(u) {
    // lazy deep clone shortcut
    const copy = JSON.parse(JSON.stringify(u));

    // console.log("saved", copy);
}
