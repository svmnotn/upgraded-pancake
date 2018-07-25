import React from 'react';
import '../styles/App.css';
import axios from 'axios';

class Table extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            heading:"Default Heading",
            dice: "1d2",
            results: []
        }
        this.getFnct = this.getFnct.bind(this);
    }


    getFnct() {
        axios.get("http://localhost:8000/table/static").then((response) => {
            console.log(response);
            this.setState ({
                heading: response.data.heading,
                dice: response.data.dice.amount + "d" + response.data.dice.size,
                results: response.data.results,
            })
        })
    }

    /*  var lines = this.state.testLines.map(function(line, i) {
    // This is just an example - your return will pull information from `line`
    // Make sure to always pass a `key` prop when working with dynamic children: https://facebook.github.io/react/docs/multiple-components.html#dynamic-children
    return (
      <div key={i}>I am a line!</div>
    );
  });*/
    render () {
        var lines = this.state.results.map(function(result, i) {
            return (
                <tr key={result.roll}>
                    <td>{result.roll}</td>
                    <td className="leftAlign">{result.value}</td>
                </tr>
            )
        })

        return (
            <div>
                <h1 onClick={this.getFnct}>Testing Table</h1>
                <hr/>
                <div style={{marginLeft: 'auto', marginRight: 'auto', width:'100%'}}>
                    <table>
                        <tbody>
                            <tr>
                                <th>{this.state.dice}</th>
                                <th className='leftAlign'>{this.state.heading}</th>
                            </tr>
                            {lines}
                        </tbody>
                    </table>

                </div>
            </div>
        )
    }
}

export default Table;
