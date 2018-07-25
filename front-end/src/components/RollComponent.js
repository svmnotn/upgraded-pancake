import React from 'react';
import '../styles/App.css';
import { Link } from 'react-router-dom';

import Table from './Table';
class Roll extends React.Component {
    render() {
        return(
            <div>
                <Table/>
                <div className="btnRow">
                    <Link to="/">
                        <button>Return</button>
                    </Link>
                </div>
            </div>
        )
    }
}

export default Roll;
