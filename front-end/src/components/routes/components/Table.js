import React from 'react';
import '../../../styles/App.css';
import {Link, withRouter } from 'react-router-dom';

class Table extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            tableResult: this.getResult(),
        }

        this.getResult = this.getResult.bind(this);
    }

    getResult () {
        let temp = this.props.dice.split("d");
        const range = temp[0] * temp[1];
        const value = Math.floor(Math.random() * (range + 1 - 1) + 1);

        for (let i = 0; i < this.props.results.length; i++) {
            if (value === this.props.results[i].roll) {
                console.log(this.props.results[i].value);
                return this.props.results[i].value;
            }

            if (typeof this.props.results[i].roll === 'string') {
                temp = this.props.results[i].roll.split("-");
                if (value >= temp[0] && value <= temp[1]){
                    console.log(this.props.results[i].value);
                    return this.props.results[i].value;
                }
            }
        }
        return "Error, value can not be found!";
    }

    reroll() {
        this.setState({
            tableResult: this.getResult(),
        })
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
