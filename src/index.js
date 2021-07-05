const fetch = require("node-fetch");
const nws = require("./nws");

module.exports.list = async function(lat, lon) {
  if( typeof lat !== "number" )
    throw new Error("required number parameter: lat");

  if( typeof lon !== "number" )
    throw new Error("required number parameter: lon");

  const response = await fetch(`https://api.weather.gov/points/${lat},${lon}/forecast`);
  const json = await response.json();

  if( !response.ok ) {
    throw new Error("Failed to fetch weather: " + JSON.stringify(json,null,2));
  }

  return nws.interpret(json);
};

module.exports.human = function(period) {
  return period.humanKey + ": The weather will be " + period.humanValue;
};

module.exports.get_now = function(periods) {
  return periods[0];
};

module.exports.get_best = function(periods) {
  let best = periods[0];

  for( const r of periods ) {
    if( r.score < best.score ) {
      best = r;
    }
  }

  return best;
};
