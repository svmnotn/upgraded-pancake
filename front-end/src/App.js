import React from 'react';
import { Link } from 'react-router-dom';
import './styles/App.css';

class App extends React.Component {
  render() {
    return (
      <div style = {{marginTop: '10%'}}>
        <h1>Upgraded Pancakes</h1>
        <p>Roll your own pancake-styled table today! Click on the roll button to roll one of our default tables. If you want to make your own table, click on the generate table button.</p>
        <div className = "btnRow">
          <div style = {{marginRight: '5vw'}}>
            <Link to = "/choose">
              <button>Roll</button>
            </Link>
          </div>
          <Link to = "/edit">
            <button> Generate Table </button>
          </Link>
        </div>
      </div>
    )
  }
}

export default App;
