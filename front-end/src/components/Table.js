import React from 'react';
import '../styles/App.css';

class Table extends React.Component {
    render () {
        return (
            <div>
                <h1>O Class Star</h1>
                <hr/>
                <div style={{marginLeft: 'auto', marginRight: 'auto', width:'100%'}}>
                    <table>
                        <tr>
                            <th>d100</th>
                            <th className='leftAlign'>Orbital Bodies</th>
                        </tr>
                        <tr>
                            <td>1</td>
                            <td className='leftAlign'>Value</td>
                        </tr>
                        <tr>
                            <td>2</td>
                            <td className='leftAlign'>Value2</td>
                        </tr>
                        <tr>
                            <td>3</td>
                            <td className='leftAlign'>Value3</td>
                        </tr>
                        <tr>
                            <td>4</td>
                            <td className='leftAlign'>Value4</td>
                        </tr>
                        <tr>
                            <td className="lastRow">5</td>
                            <td className='leftAlign lastRow'>Value5</td>
                        </tr>
                    </table>
                </div>
            </div>
        )
    }
}

export default Table;
