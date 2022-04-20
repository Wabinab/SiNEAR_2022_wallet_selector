import React, {useEffect, useState} from 'react';
import PropTypes from 'prop-types';
import Big from 'big.js';


const render_based_on_page = (current_page, get_single_message)  => {
  const [message, setMessage] = useState({
    premium: '',
    money: '',
    sender: '',
    text: '',
    datetime: ''
  });

  get_single_message(
    parseInt(current_page.split('/')[1])
  ).then((message) => {
    setMessage(message[0]);
  });

  // contract.get_single_message({
  //   index: parseInt(current_page.split('/')[2])
  // }).then((message) => {
  //   setMessage(message);
  // });

  return (
    <div>
      <h1>Signed by: {message.sender}</h1>
      <p>On {message.datetime}</p>

      <br/><br/>
      <strong>Money Attached:</strong>
      <p>{message.money} Ⓝ</p>

      <br/><br/>
      <strong>Message:</strong>
      <p>{message.text}</p>

      <br />
      <button><a href="/">Home</a></button>
    </div>
  );
  
}

export default function Form({ onSubmit, current_page, get_single_message }) {
  if (current_page == "/") {
    var account = window.account;

    return (
      <form onSubmit={onSubmit}>
        <fieldset id="fieldset">
          <p>Sign the guest book, { account.account_id }!</p>
          <p className="highlight">
            <label htmlFor="message">Message:</label>
            <input
              autoComplete="off"
              autoFocus
              id="message"
              required
            />
          </p>
          <p>
            <label htmlFor="donation">Donation (optional):</label>
            <input
              autoComplete="off"
              defaultValue={'0'}
              id="donation"
              max={Big(account.amount).div(10 ** 24).toString()}
              // max="10"
              min="0"
              step="0.01"
              type="number"
            />
            <span title="NEAR Tokens">Ⓝ</span>
          </p>
          <button type="submit">
            Sign
          </button>
        </fieldset>
      </form>
    );
  } else {
    return render_based_on_page(current_page, get_single_message)
    // return 'hi'
  }
}

Form.propTypes = {
  onSubmit: PropTypes.func.isRequired,
  current_page: PropTypes.string.isRequired,

  // currentUser: PropTypes.shape({
  //   accountId: PropTypes.string.isRequired,
  //   balance: PropTypes.string.isRequired
  // })
};
