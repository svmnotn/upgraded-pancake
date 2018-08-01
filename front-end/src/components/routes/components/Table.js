import React from 'react';
import '../../../styles/App.css';
import axios from 'axios';
import {Link, withRouter } from 'react-router-dom';

class Table extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            tableResult: this.getResult(),
        }
        this.getResult = this.getResult.bind(this);
    }

    getResult() {
        console.log(this.props.results);
        axios.post('http://localhost:8000/table', {
            heading: this.props.heading,
            dice: this.props.dice,
            results:  this.props.results,
        }).then ((response) => {
            this.setState({
                tableResult: response.data.value
            })
        }).catch(function(error){
            console.log(error);
        })
    }

    reroll() {
        this.getResult();
    }

    render () {
        let createRows = this.props.results.map(function(result, i) {
            return (
                <tr key={result.roll}>
                    <td>{result.roll}</td>
                    <td className="leftAlign">{result.value}</td>
                </tr>
            )
        })

        return (
            <div>
                <h1>{this.props.title}</h1>
                <hr/>
                <div style={{marginLeft: 'auto', marginRight: 'auto', width:'100%'}}>
                    <table>
                        <tbody>
                            <tr>
                                <th>{this.props.dice}</th>
                                <th className='leftAlign'>{this.props.heading}</th>
                            </tr>
                            {createRows}
                        </tbody>
                    </table>
                </div>

                <p className="result">{this.state.tableResult}</p>

                <div className="btnRow">
                    <button style= {{marginRight: '5%',}}
                            onClick={()=>{this.reroll()}}>Reroll</button>
                    <Link to="/Choose">
                        <button>Return</button>
                    </Link>
                </div>
            </div>
        )
    }
}

export default withRouter(Table);
