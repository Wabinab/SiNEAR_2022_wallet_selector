import React from 'react';
import PropTypes from 'prop-types';

export default function Messages({ messages }) {
  return (
    <>
      <h2>Messages</h2>
      <div className="articles">
        <ul className="articles">
          {messages.map((message, i) =>
            // TODO: format as cards, add timestamp
            <li className="articles" key={i}>
              <p key={i} className={message.premium ? 'is-premium' : ''}>
                <h4>{message.sender}</h4>
                (on <u className="articles">{message.date}</u>)<br/><br />
                <strong>Message: </strong>{message.text}
              </p>
            </li>
          ).reverse()}
        </ul>
      </div>
    </>
  );
}

Messages.propTypes = {
  messages: PropTypes.array
};
