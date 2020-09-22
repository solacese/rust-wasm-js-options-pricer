<script>
  import _ from 'lodash';

  export let black_scholes_pricer;
  export let optionMarketData;

  let time_taken = 0;
  let call_prices = [];
  let put_prices = [];

  function get_bs_price(spot, expiry, option_data, option_direction) {
    let spot_array = [],
      strike_array = [],
      years_array = [],
      rfr_array = [],
      vol_array = [],
      dividend_array = [];
    _.each(option_data, (option) => {
      spot_array.push(spot);
      strike_array.push(option.strike);
      years_array.push(expiry);
      rfr_array.push(option.risk_free_rate);
      vol_array.push(option.volatility);
      dividend_array.push(option.dividend_yield);
    });

    let bs_price = [];
    if (option_direction) {
      let currTime = new Date();
      bs_price = black_scholes_pricer.bs_call(spot_array, strike_array, years_array, rfr_array, vol_array, dividend_array);
      time_taken = new Date() - currTime;
    } else {
      let currTime = new Date();
      bs_price = black_scholes_pricer.bs_put(spot_array, strike_array, years_array, rfr_array, vol_array, dividend_array);
      time_taken += new Date() - currTime;
    }

    let option_prices = [];

    for (let i = 0; i < bs_price.length; i++) {
      if (!isNaN(bs_price[i])) {
        if (bs_price[i] < 0) bs_price[i] = 0.0;
        let option_price = { strike: strike_array[i], price: bs_price[i].toFixed(2) };
        option_prices.push(option_price);
      }
    }

    return option_prices;
  }

  $: {
    call_prices = _.sortBy(get_bs_price(optionMarketData.spot, optionMarketData.option_expiry, _.filter(optionMarketData.option_data, 'direction'), 1), 'strike');

    put_prices = _.sortBy(get_bs_price(optionMarketData.spot, optionMarketData.option_expiry, _.filter(optionMarketData.option_data, ['direction', 0])), 'strike');
  }
</script>

<style>
  table,
  th,
  td {
    border: 1px solid #00c895;
    color: #00c895;
  }
</style>

<table>
  <tr>
    <th colspan="3">Options Table for <b>{optionMarketData.ticker}</b> {optionMarketData.spot.toFixed(2)}<br /> Expiry: {optionMarketData.option_expiry.toFixed(2)}</th>
  </tr>
  <tr>
    <th>Call Price</th>
    <th>Strike</th>
    <th>Put Price</th>
  </tr>
  {#each call_prices as call_price, i}
    <tr>
      <td>{call_price.price}</td>
      <td>{call_price.strike}</td>
      <td>{put_prices[i].price}</td>
    </tr>
  {/each}
  <th colspan="3">Option Prices calculated in: <i>{time_taken}ms</i></th>
</table>
