const common = require("./common.js");

module.exports.interpret = function(json) {
  const periods = json.properties.periods;

  return periods.map(period => ({
    score: score(period),
    humanKey: period.name,
    humanValue: period.detailedForecast
  }));
};

function score(period) {
  return common.scoreTemperature(period.temperature) + scoreConditions(period.detailedForecast);
}

function scoreConditions(str) {
  if( /[Rr]ain|[Ss]now|[Pp]recipitation|[Ss]howers|[Dd]rizzle/.test(str) )
    return 100;

  return 0;
}
