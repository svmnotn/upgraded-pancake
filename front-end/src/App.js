import React from 'react';
import './styles/App.css';
import axios from 'axios';

import { Link } from 'react-router-dom';

class App extends React.Component {
  constructor() {
    super();
    this.state = {
        savedTables: ["static"],
    }
    this.getSavedTables = this.getSavedTables.bind(this);
}

componentDidMount() {
    this.getSavedTables();
}

getSavedTables() {
    axios.get("/table/all/name").then((response) => {
        let tempTables = this.state.savedTables.concat(response.data);
        this.setState({
            savedTables: tempTables,
        })
    })
}

  render() {
    return (
      <div style = {{marginTop: '10%'}}>
        <h1>Upgraded Pancakes</h1>
        <p>Roll your own pancake-styled table today! Click on the roll button to roll one of our default tables. If you want to make your own table, click on the generate table button.</p>
        <div className = "btnRow">
          <div style = {{marginRight: '5vw'}}>
            <Link to = {{ pathname: "/choose",
                          state: { savedTables: this.state.savedTables}
                        }}>
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
