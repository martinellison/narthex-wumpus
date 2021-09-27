console.log("starting scripts");
console.log("\tversion: " + navigator.appVersion);
console.log("\tuser agent: " + navigator.userAgent);
window.addEventListener("load", function () {
    console.log("Page loaded");
    invoke("Instructions");
});
var setTag = function (tag, text) {
    var s = document.getElementById(tag);
    if (!s) console.error("no tag called '" + tag + "'");
    else s.innerHTML = text;
};
var previous = '';
// interface type is {{interface_type}}
{% match interface_type %}
{% when  narthex_engine_trait:: InterfaceType:: PC %}
// code for PC platform
var invoke = function (arg) {
    console.log("\ninvoking from PC with " + JSON.stringify(arg));
    window.webkit.messageHandlers.external.postMessage(JSON.stringify(arg));
};
{% when narthex_engine_trait:: InterfaceType:: Android %}
// code for Android
var invoke = function (arg) {
    console.log("\ninvoking from Android with " + JSON.stringify(arg));
    wumpus.execute(JSON.stringify(arg));
    console.log("getting response...");
    var response_json = wumpus.last_response_json();
    console.log("execute done, response" + response_json);
    respond( response_json);
};
{% endmatch %}
var oldEventListeners = Array(3);
// process response
var respond = function (response_str) {
    console.log("handling response " + response_str);
    var response = JSON.parse(response_str);
    setTag("msgs", response.msgs);
    for (i = 0; i < 3; i++) {
        let tunnel = response.tunnels[i];
        var tag = "move" + i;
        var s = document.getElementById(tag);
        if (!s) console.error("no tag called '" + tag + "'");
        var svalue = "room " + tunnel;
        s.textContent = svalue;
        var newEventListener = function () {
            var t = tunnel;
            console.log("go to " + t);
            invoke({ Move: t })
        };
        console.log('set up move for ' + tunnel);
        if (oldEventListeners[i])
            s.removeEventListener('click', oldEventListeners[i], false);
        oldEventListeners[i] = newEventListener;
        s.addEventListener('click', newEventListener, false);
    }
    console.log("processed response");
}
var shoot = function () {
    console.log("shooting arrow");
    var rooms = [];
    var ids = ['arrow0', 'arrow1', 'arrow2', 'arrow3', 'arrow4'];
    for (id in ids) {
        var ii = document.getElementById(ids[id]);
        if (!ii) console.error("no tag called '" + id + "'");
        var v = parseInt(ii.value);
       // console.log("value " + v + " type " + typeof v);
        if (!isNaN(v)) {
            rooms.push(v);
        }
        ii.value = "";
    }
    invoke({ Shoot: rooms });
}
console.log("compile ok");
