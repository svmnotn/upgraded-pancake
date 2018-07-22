import React from 'react';
import ReactDOM from 'react-dom';
import {
    BrowserRouter as Router,
    Route,
    Switch,
  } from 'react-router-dom';
import registerServiceWorker from './registerServiceWorker';

import Choose from './components/ChooseComponent';
import Home from './components/HomeComponent';
import App from './App';
import './styles/index.css';

ReactDOM.render(
    <Router>
        <Switch>
            <Route exact path="/" component={Home}/>
            <Route path="/choose" component={Choose}/>
        </Switch>
    </Router>
, document.getElementById('root'));
registerServiceWorker();
