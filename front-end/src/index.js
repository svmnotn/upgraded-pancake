import React from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter, Route, Switch } from 'react-router-dom';
import registerServiceWorker from './registerServiceWorker';

import './styles/index.css';
import Home from './App';
import Choose from './components/routes/ChooseComponent';
import Roll from './components/routes/RollComponent';
import Edit from './components/routes/EditComponent';

//<Route exact render={(props) => (<Home testStr="hi" {...props}/>)} path="/roll" component={Roll}/>
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
