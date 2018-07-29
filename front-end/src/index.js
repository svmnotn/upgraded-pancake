import React from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter, Route, Switch } from 'react-router-dom';
import registerServiceWorker from './registerServiceWorker';

import './styles/index.css';
import Home from './App';
import Choose from './components/routes/ChooseComponent';
import Create from './components/routes/CreateComponent';
import Roll from './components/routes/RollComponent';

//<Route exact render={(props) => (<Home testStr="hi" {...props}/>)} path="/roll" component={Roll}/>
ReactDOM.render(
    <BrowserRouter>
        <Switch>
            <Route exact path="/" component={Home}/>
            <Route path="/choose" component={Choose}/>
            <Route path="/create" component={Create}/>
            <Route path="/roll" component={Roll}/>
        </Switch>
    </BrowserRouter>
, document.getElementById('root'));
registerServiceWorker();
