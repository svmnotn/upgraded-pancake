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


/*
 GET / (index)
    => PUT /table/<name> application/json (put)
    => GET /table/<name> (get)
    => DELETE /table/<name> (delete)
    => GET /table/all/name (table_name)
    => GET /table/all/data (table_data)
    => GET /table/<name>/roll (roll_saved)
    => POST /table application/json (roll)
    => GET /table/static (static_tables)
    => POST /table/validate application/json (validate)
    => GET /<file..> [2] (get)
*/
