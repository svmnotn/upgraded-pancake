import React from 'react';
import './styles/App.css';

import Home from './components/HomeComponent';
class App extends React.Component {
  constructor() {
    super();
    this.state = {
      bgDisplay: "",
    }
    this.changePage = this.changePage.bind(this);
  }

  changePage () {
    this.setState({
      bgDisplay: 'none',
    })
  }

  render() {
    return (
      <div>
        <div style={{display: this.state.bgDisplay}}>
          <Home changePage={this.changePage}/>
        </div>
      </div>
    );
  }
}

export default App;
