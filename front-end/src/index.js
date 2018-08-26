import React from 'react';
import ReactDOM from 'react-dom';
import './styles/index.css';
import registerServiceWorker from './registerServiceWorker';

import { BrowserRouter, Route, Switch } from 'react-router-dom';
import Home from './App';
import Choose from './components/routes/ChooseComponent';
import Roll from './components/routes/RollComponent';
import Edit from './components/routes/EditComponent';

ReactDOM.render(
    <BrowserRouter>
        <Switch>
            <Route exact path="/" component={Home}/>
            <Route path="/choose" component={Choose}/>
            <Route path="/roll" component={Roll}/>
            <Route path="/edit" component={Edit}/>
        </Switch>
    </BrowserRouter>
, document.getElementById('root'));
registerServiceWorker();

