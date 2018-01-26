'use strict';

const fetch = require('node-fetch');

module.exports.list = async function() {
  const response = await fetch('https://api.weather.gov/points/35.7796,-78.6382/forecast');
  const json = await response.json();
  const periods = json.properties.periods;

  return periods.map(p => Object.assign({
    score: scoreHeat(p.temperature) + scoreCold(p.temperature) + scoreConditions(p.detailedForecast) + scoreDaytime(p.isDaytime) + scoreFuture(p.number),
    humanKey: p.name,
    humanValue: p.detailedForecast
  }));
};

module.exports.human = function(list) {
  let first_best = list[0];
  let inflection = false;
  let best = list[0];

  for( let i = 1; i < list.length; i++ ) {
    if( list[i].score < best.score )
      best = list[i];

    if( list[i].score < first_best.score && !inflection )
      first_best = list[i];
    else
      inflection = true;
  }

  const results = [];
  if( first_best === list[0] && first_best.score < 50 ) {
    results.push(first_best);
    if( first_best !== best && first_best.score < 25 )
      results.push(best);
  }

  return results.map(r => 'The weather ' + r.humanKey + ' will be ' + r.humanValue);
};

function scoreHeat(temp_f) {
  return Math.max(0, temp_f - 75);
}

function scoreCold(temp_f) {
  return Math.max(0, 68 - temp_f);
}

function scoreConditions(str) {
  if( /[Rr]ain/.test(str) || /[Ss]now/.test(str) || /[Pp]recipitation/.test(str) )
    return 100;

  return 0;
}

function scoreDaytime(is_daytime) {
  if( !is_daytime )
    return 25;
  else
    return 0;
}

function scoreFuture(number) {
  return number;
}

async function main() {
  module.exports.human(await module.exports.list()).forEach(s => console.log(s+'\n')); // eslint-disable-line no-console
}

main();
