// Basic Title 
import {TitleProps} from '../interfaces/AtomProps';

export default function Title(props: TitleProps) {
    return (
        <h1 className="text-3xl font-bold title">{props.title}</h1>
    );
}
