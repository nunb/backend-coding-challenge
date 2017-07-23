"use strict";
var suggestion = document.getElementById("suggestion");
var suglat = document.getElementById("suglat");
var suglong = document.getElementById("suglong");
var sugradius = document.getElementById("sugradius");
var loctab = document.getElementById("loctab");
var map = document.getElementById("map");
var msg = document.getElementById("msg");
var suggestDelay = null;
var mapRect = {
    w: -171,
    e: -48,
    n: 83,
    s: 25,
};
function doSearchSoon() {
    if (suggestDelay) {
        clearTimeout(suggestDelay);
    }
    suggestDelay = setTimeout(function(){
        doSearch();
    }, 90);
}
var inputFields = [suggestion, suglat, suglong, sugradius];
for (var i = 0; i<inputFields.length; i++) {
    inputFields[i].addEventListener("input", doSearchSoon);
    inputFields[i].addEventListener("keydown", function(e){
        if (e.keyCode == 13) {
            doSearch();
        }
    });
}
map.addEventListener("mousedown", updateCoords);
function updateCoords(e) {
    suglat.value = (mapRect.n - (mapRect.n - mapRect.s) * (e.pageY - map.offsetTop) / map.offsetHeight).toFixed(6);
    suglong.value = (mapRect.w - (mapRect.w - mapRect.e) * (e.pageX - map.offsetLeft) / map.offsetWidth).toFixed(6);
    doSearch();
}
function doSearch() {
    if (suggestDelay) {
        clearTimeout(suggestDelay);
        suggestDelay = null;
    }
    msg.style.display = "block";
    msg.textContent = "Searching for '" + suggestion.value + "'";
    var xhr = new XMLHttpRequest();
    var query = "/suggestions?q=" + encodeURIComponent(suggestion.value);
    if (suglat.value) query += "&latitude=" + suglat.value;
    if (suglong.value) query += "&longitude=" + suglong.value;
    if (sugradius.value) query += "&radius=" + sugradius.value;
    xhr.open("GET", query, true);
    xhr.addEventListener("load", function(){
        var data = JSON.parse(this.responseText);
        if (data.err) {
            msg.textContent = data.err;
            msg.style.display = "block";
            loctab.style.display = "none";
        }
        else if (data.suggestions) {
            if (data.suggestions.length) {
                msg.style.display = "none";
                loctab.style.display = "block";
                while (loctab.rows.length > 1) loctab.deleteRow(-1);
                var marks = document.getElementsByClassName("mark");
                for (var i = marks.length - 1; i>-1; i--) {
                    document.body.removeChild(marks[i]);
                }
                for (var i = 0; i<data.suggestions.length; i++) {
                    var mark = document.createElement("div");
                    var row = loctab.insertRow();
                    (function(mark, row){
                        mark.addEventListener("mousedown", updateCoords);
                        mark.className = "mark";
                        mark.style.left = (map.offsetLeft + map.offsetWidth * (data.suggestions[i].longitude - mapRect.w) / (mapRect.e - mapRect.w))+"px";
                        mark.style.top = (map.offsetTop + map.offsetHeight * (mapRect.n - data.suggestions[i].latitude) / (mapRect.n - mapRect.s))+"px";
                        document.body.appendChild(mark);

                        row.insertCell().textContent = data.suggestions[i].name;
                        row.insertCell().textContent = data.suggestions[i].latitude;
                        row.insertCell().textContent = data.suggestions[i].longitude;
                        row.insertCell().textContent = data.suggestions[i].score;
                        function mouseover() {
                            row.style.background = "#ff8";
                            mark.style.background = "#f00";
                            mark.style.zIndex = "1";
                        }
                        function mouseout() {
                            row.style.background = "";
                            mark.style.background = "#000";
                            mark.style.zIndex = "0";
                        }
                        mark.addEventListener("mouseover", mouseover);
                        mark.addEventListener("mouseout", mouseout);
                        row.addEventListener("mouseover", mouseover);
                        row.addEventListener("mouseout", mouseout);
                    })(mark, row);
                }
            } else {
                msg.textContent = "No results found";
                msg.style.display = "block";
                loctab.style.display = "none";
            }
        }
    });
    xhr.send();
}
