import { type NextPage } from "next";
import Head from "next/head";
import Link from "next/link";
import Footer from "../components/Footer";
import Header from "../components/Header";
import Hero from "../components/Hero";
// import index.module.css from current folder

const Home: NextPage = () => {
  return (
    <>
      <Head>
        <title>Rumarkio : Bookmarking and Research Assistant</title>
        <meta name="description" content="Rumarkio is bookmaking application that helps you save your research" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className="bg-background-primary-light">
      <Header/> 
      <Hero/>
      <Footer/>
      </main>
    </>
  );
};

export default Home;
