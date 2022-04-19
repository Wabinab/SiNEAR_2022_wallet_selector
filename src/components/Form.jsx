import React, {useEffect, useState} from 'react';
import PropTypes from 'prop-types';
import Big from 'big.js';


const render_based_on_page = (current_page, contract)  => {
  const [message, setMessage] = useState({
    premium: '',
    money: '',
    sender: '',
    text: '',
    datetime: ''
  });
  contract.get_single_message({
    index: parseInt(current_page.split('/')[2])
  }).then((message) => {
    setMessage(message);
  });

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

export default function Form({ onSubmit, currentUser, current_page, contract }) {
  if (current_page == "/") {
    return (
      <form onSubmit={onSubmit}>
        <fieldset id="fieldset">
          <p>Sign the guest book, { currentUser.accountId }!</p>
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
              max={Big(currentUser.balance).div(10 ** 24)}
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
    return render_based_on_page(current_page, contract)
  }
}

Form.propTypes = {
  onSubmit: PropTypes.func.isRequired,
  currentUser: PropTypes.shape({
    accountId: PropTypes.string.isRequired,
    balance: PropTypes.string.isRequired
  })
};
