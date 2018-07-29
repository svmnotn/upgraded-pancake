import React from 'react';
import '../../styles/App.css';
import { Link } from 'react-router-dom';

import Table from './components/Table';
class Create extends React.Component {
    render() {
        return(
            <div>
                <Table title="Default Table Name"
                       heading="Item Categories"
                       dice= "#d#"
                       results={[]}/>
                <div className='container'>
                    <Link to="/">
                        <button className="squareBtn"><i className="fa fa-times"></i></button>
                    </Link>
                    <div style={{marginLeft: '30%', marginRight: '45%'}}></div>
                    <Link to = {{ pathname: "/roll",
                                  state: {
                                  title: "Default Table Name",
                                  heading: "Item Categories",
                                  dice: "#d#",
                                  results: [],
                                }}}>
                        <button className="squareBtn"><i className="fa fa-check"></i></button>
                    </Link>
                </div>
            </div>
        )
    }
}

export default Create;
