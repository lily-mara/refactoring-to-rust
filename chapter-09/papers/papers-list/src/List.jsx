import { useEffect, useState } from 'react'
const wasm = await import('papers')

const List = () => {
    const [entries, setEntries] = useState([])
    const [page, setPage] = useState(0)
    useEffect(() => {
        if(wasm){        
            wasm.paper_search({"term":"type", "page": page, "limit": 10}).then(
                (result)=>setEntries(result.entry),
                (error)=>console.error(error))
        }
      }, [page])
       
    return (
        <div>
        <ul>
            {entries?.map((v, i) => {
             return <li key={i}>
                <a href={`${v.id}`} target='_blank'>{v.title}</a>
             </li>
            })}
        </ul>
        <button onClick={() => setPage((page) => page + 1)}>More</button>
        </div>
    )
}
export default List;