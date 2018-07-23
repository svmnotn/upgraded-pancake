import React from 'react';
import '../styles/App.css';
import { Link } from 'react-router-dom';

import Table from './Table';

class Create extends React.Component {
    render() {
        return(
            <div>
                <Table/>
                <div className='container'>
                    <Link to="/">
                        <button className="squareBtn"><i className="fa fa-times"></i></button>
                    </Link>
                    <div style={{marginLeft: '30%', marginRight: '45%'}}></div>
                    <Link to="/Roll">
                        <button className="squareBtn"><i className="fa fa-check"></i></button>
                    </Link>
                </div>
            </div>
        )
    }
}

export default Create;
