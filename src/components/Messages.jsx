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
              <a href={"/" + i} className="articles">
                <p key={i} className={message.premium ? 'is-premium' : ''}>
                  <strong className='articles'>{message.sender}</strong><br/>
                  (on <u className="articles">{message.datetime}</u>)<br/><br />
                  <b>Message: </b>{message.text}
                </p>
              </a>
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
