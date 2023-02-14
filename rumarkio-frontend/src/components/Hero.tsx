import Painting from "./Painting";
import { PaintingProps } from '../interfaces/CompProps';
import style from './Hero.module.css';

// create painting this.props.
export default function Hero() {
    const paintingPropsOne: PaintingProps = {
        img: "/images/hero1.png",
        description: "Rumarkio Hero Image",
        alt_color: "#F7F7F2",
    };
    const paintingPropsTwo: PaintingProps = {
        img: "/images/hero2.png",
        description: "Rumarkio Hero Image",
        alt_color: "#F7F7F2",
    };
    const paintingPropsThree: PaintingProps = {
        img: "/images/hero3.png",
        description: "Rumarkio Hero Image",
        alt_color: "#F7F7F2",
    };

    return (
        <div className="mt-8 hero h-screen">
            <div className="hero flex flex-col items-center  h-full">
                <h1 className="font-fira-code text-center text-5xl font-bold mb-4">
                    - The Intelligent Bookmark Companion
                </h1>
                <h2 className={`font-fira-mono text-center text-xl font-medium ${style.subtitle_element}`}>
                    Streamline your research with customized bookmarks, libraries and notes.
                </h2>
                <div className="-mt-10 mb-3 md:flex flex-row justify-center mt-8">
                    <div className="items-center justify-center">
                        <Painting {...paintingPropsOne} />
                    </div>
                    <div className={`mt-20 items-center justify-center ${style.bichko_element}`} >
                        <Painting {...paintingPropsTwo} />
                    </div>
                    <div className=" items-center justify-center">
                        <Painting {...paintingPropsThree} />
                    </div>
                </div>
            </div>
        </div>
    );
}

